use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::ops::DerefMut;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::thread;
use std::thread::JoinHandle;

use either::Either;
use os_pipe::{pipe, PipeReader, PipeWriter};

use crate::ast::{Cmd, CmdOp, Expr};

use super::Interpreter;
use super::value::Value;

#[cfg(test)]
mod test;

pub type OsEnv = Vec<(String, String)>;

enum Process {
    Std(Either<Command, Child>),
    Pipe {
        lhs: Box<Process>,
        rhs: Box<Process>,
    },
    Cond {
        op: CmdOp,
        procs: Option<Box<(Process, Process)>>,
        handle: Option<JoinHandle<ExitStatus>>,
    },
}

impl Process {
    fn wait(&mut self) -> ExitStatus {
        match self {
            Process::Std(either) => {
                match either {
                    Either::Left(_) => panic!("process not spawned"),
                    Either::Right(child) => child.wait().unwrap(),
                }
            }
            Process::Pipe { lhs, rhs } => {
                lhs.wait();
                rhs.wait()
            }
            Process::Cond { handle, .. } => {
                handle.take().unwrap().join().unwrap()
            }
        }
    }

    fn set_env(&mut self, env: OsEnv) {
        match self {
            Process::Std(either) => {
                match either {
                    Either::Left(cmd) => {
                        cmd.envs(env);
                    }
                    Either::Right(_) => panic!("process already spawned"),
                }
            }
            Process::Pipe { lhs, rhs } => {
                lhs.set_env(env.clone());
                rhs.set_env(env);
            }
            Process::Cond { procs, .. } => {
                let (lhs, rhs) = procs.as_mut().unwrap().deref_mut();
                lhs.set_env(env.clone());
                rhs.set_env(env);
            }
        }
    }

    fn spawn(&mut self) {
        match self {
            Process::Std(either) => {
                match either {
                    Either::Left(cmd) => {
                        let child = cmd.spawn().unwrap();
                        *either = Either::Right(child);
                    }
                    Either::Right(_) => panic!("process already spawned"),
                }
            }
            Process::Pipe { lhs, rhs } => {
                lhs.spawn();
                rhs.spawn();
            }
            Process::Cond { procs, handle, op } => {
                let op = *op;
                let (mut lhs, mut rhs) = *procs.take().unwrap();

                lhs.spawn();

                *handle = Some(thread::spawn(move || {
                    let lhs_exit = lhs.wait();

                    let spawn_rhs = match op {
                        CmdOp::Seq => true,
                        CmdOp::Or if !lhs_exit.success() => true,
                        CmdOp::And if lhs_exit.success() => true,
                        _ => false,
                    };

                    if spawn_rhs {
                        rhs.spawn();
                        rhs.wait()
                    } else {
                        lhs_exit
                    }
                }));
            }
        }
    }
}

enum Stream {
    Inherit,
    Null,
    File(File),
    PipeReader(PipeReader),
    PipeWriter(PipeWriter),
}

impl Clone for Stream {
    fn clone(&self) -> Self {
        match self {
            Stream::Inherit => Stream::Inherit,
            Stream::Null => Stream::Null,
            Stream::File(_) => panic!("clone file"),
            Stream::PipeReader(r) => Stream::PipeReader(r.try_clone().unwrap()),
            Stream::PipeWriter(w) => Stream::PipeWriter(w.try_clone().unwrap()),
        }
    }
}

impl Into<Stdio> for Stream {
    fn into(self) -> Stdio {
        match self {
            Stream::Inherit => Stdio::inherit(),
            Stream::Null => Stdio::null(),
            Stream::File(file) => Stdio::from(file),
            Stream::PipeReader(pipe_reader) => pipe_reader.into(),
            Stream::PipeWriter(pipe_writer) => pipe_writer.into(),
        }
    }
}

impl Interpreter {
    pub fn run_cmd_pipe(&mut self, cmd: Cmd, env: OsEnv) {
        let mut cmd = self.build_cmd(cmd, Stream::Null, Stream::Inherit, Stream::Inherit);
        cmd.set_env(env);
        cmd.spawn();
        cmd.wait();
    }

    pub fn run_cmd_capture(&mut self, cmd: Cmd, env: OsEnv) -> String {
        let (mut r, w) = pipe().unwrap();

        let mut cmd = self.build_cmd(cmd, Stream::Null, Stream::PipeWriter(w), Stream::Inherit);
        cmd.set_env(env);
        cmd.spawn();
        cmd.wait();

        let mut out = String::new();
        r.read_to_string(&mut out).unwrap();

        out
    }

    fn build_cmd(&mut self, cmd: Cmd, mut stdin: Stream, mut stdout: Stream, mut stderr: Stream) -> Process {
        match cmd {
            Cmd::Atom(segments) => {
                let mut segments = self.raster_segments(segments);

                let mut cmd = Command::new(segments.remove(0));
                cmd.args(segments);

                cmd.stdin(stdin);
                cmd.stdout(stdout);
                cmd.stderr(stderr);

                Process::Std(Either::Left(cmd))
            }
            Cmd::Op(lhs, op, rhs) if [CmdOp::OutPipe, CmdOp::ErrPipe, CmdOp::AllPipe].contains(&op) => {
                let (r, w) = pipe().unwrap();

                let (out, err) = match op {
                    CmdOp::OutPipe => (Stream::PipeWriter(w), Stream::Null),
                    CmdOp::ErrPipe => (Stream::Null, Stream::PipeWriter(w)),
                    CmdOp::AllPipe => (Stream::PipeWriter(w.try_clone().unwrap()), Stream::PipeWriter(w)),
                    _ => unreachable!()
                };

                let lhs = self.build_cmd(*lhs, stdin, out, err);
                let rhs = self.build_cmd(*rhs, Stream::PipeReader(r), stdout, stderr);

                Process::Pipe {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }
            }
            Cmd::Op(lhs, op, rhs) if [CmdOp::And, CmdOp::Or, CmdOp::Seq].contains(&op) => {
                let (in_1, in_2) = (stdin.clone(), stdin);
                let (out_1, out_2) = (stdout.clone(), stdout);
                let (err_1, err_2) = (stderr.clone(), stderr);

                let lhs = self.build_cmd(*lhs, in_1, out_1, err_1);
                let rhs = self.build_cmd(*rhs, in_2, out_2, err_2);

                Process::Cond {
                    op,
                    procs: Some(Box::new((lhs, rhs))),
                    handle: None,
                }
            }
            Cmd::Op(lhs, op, rhs) if [CmdOp::OutWrite, CmdOp::ErrWrite, CmdOp::AllWrite, CmdOp::OutAppend, CmdOp::ErrAppend, CmdOp::AllAppend, CmdOp::Read].contains(&op) => {
                let path = self.cmd_to_path(*rhs);

                let mut file = File::with_options();

                let file = match op {
                    CmdOp::OutWrite | CmdOp::ErrWrite | CmdOp::AllWrite => file.create(true).write(true).truncate(true),
                    CmdOp::OutAppend | CmdOp::ErrAppend | CmdOp::AllAppend => file.create(true).append(true),
                    CmdOp::Read => file.read(true),
                    _ => unreachable!(),
                };

                let file = file.open(&path).unwrap();

                match op {
                    CmdOp::Read => stdin = Stream::File(file),
                    CmdOp::OutWrite | CmdOp::OutAppend => stdout = Stream::File(file),
                    CmdOp::ErrWrite | CmdOp::ErrAppend => stderr = Stream::File(file),
                    CmdOp::AllWrite | CmdOp::AllAppend => {
                        let file_cloned = file.try_clone().unwrap();
                        stdout = Stream::File(file);
                        stderr = Stream::File(file_cloned);
                    }
                    _ => unreachable!()
                }

                self.build_cmd(*lhs, stdin, stdout, stderr)
            }
            _ => unreachable!()
        }
    }

    fn raster_segments(&mut self, segments: Vec<Vec<Expr>>) -> Vec<String> {
        let home_dir = dirs::home_dir().expect("bad home dir").display().to_string();

        let mut out = Vec::new();

        for segment in segments {
            let mut vals: Vec<Value> = segment.into_iter().map(|expr| self.eval(expr)).collect();

            vals.iter_mut().for_each(|val| {
                if let Value::String(str) = val {
                    *str = str.replace("~", &home_dir);
                }
            });

            out.append(&mut cross_product(vals));
        }

        out
    }

    fn cmd_to_path(&mut self, cmd: Cmd) -> String {
        let segments = if let Cmd::Atom(segments) = cmd {
            segments
        } else {
            panic!("expected atom command");
        };

        let mut segments = self.raster_segments(segments);

        if segments.len() != 1 {
            panic!("expected one segment");
        }

        segments.remove(0)
    }
}

fn cross_product(mut vals: Vec<Value>) -> Vec<String> {
    let mut out = vec![String::from("")];

    vals.reverse();

    for val in vals {
        match val {
            Value::Vec(prefixes) => {
                let mut out_tmp = Vec::new();

                for prefix in RefCell::borrow(&prefixes).iter() {
                    let prefix = prefix.to_string();

                    for s in &out {
                        out_tmp.push(format!("{}{}", &prefix, s));
                    }
                }

                out = out_tmp;
            }
            _ => {
                let prefix = val.to_string();
                for s in &mut out {
                    s.insert_str(0, &prefix);
                }
            }
        }
    }

    out
}
