use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, ExitStatus, Stdio};
use std::rc::Rc;
use std::thread;
use std::thread::JoinHandle;

use itertools::Itertools;
use os_pipe::{pipe, PipeReader, PipeWriter};

use crate::ast::{Cmd, CmdOp, Expr, Prog, Stmt};

#[cfg(test)]
mod test;

pub struct Interpreter {}

enum Process {
    Std(Child),
    Pipe {
        lhs: Box<Process>,
        rhs: Box<Process>,
    },
    And {
        lhs: Box<Process>,
        t: JoinHandle<ExitStatus>,
    },
}

enum Stream {
    Inherit,
    Null,
    PipeReader(PipeReader),
    PipeWriter(PipeWriter),
}

impl Clone for Stream {
    fn clone(&self) -> Self {
        match self {
            Stream::Inherit => Stream::Inherit,
            Stream::Null => Stream::Null,
            Stream::PipeReader(r) => Stream::PipeReader(r.try_clone().unwrap()),
            Stream::PipeWriter(w) => Stream::PipeWriter(w.try_clone().unwrap()),
            _ => panic!("unexpected stream"),
        }
    }
}

impl Into<Stdio> for Stream {
    fn into(self) -> Stdio {
        match self {
            Stream::Inherit => Stdio::inherit(),
            Stream::Null => Stdio::null(),
            Stream::PipeReader(pipe_reader) => pipe_reader.into(),
            Stream::PipeWriter(pipe_writer) => pipe_writer.into(),
        }
    }
}

impl Process {
    fn wait(&mut self) -> ExitStatus {
        match self {
            Process::Std(child) => child.wait().unwrap(),
            Process::Pipe { lhs, rhs } => {
                lhs.wait();
                rhs.wait()
            }
            Process::And { lhs, t } => {
                t.join().unwrap()
            }
            _ => todo!()
        }
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn run(&mut self, prog: Prog) {
        for stmt in prog.into_iter() {
            self.run_stmt(stmt);
        }
    }

    fn run_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Cmd(cmd) => {
                let mut cmd = self.run_cmd(cmd, Stream::Null, Stream::Inherit, Stream::Inherit);
                cmd.wait();
            }
            _ => todo!(),
        };
    }

    fn run_cmd(&mut self, cmd: Cmd, stdin: Stream, stdout: Stream, stderr: Stream) -> Process {
        match cmd {
            Cmd::Atom(segments) => {
                let mut segments = segments.into_iter().map(
                    |exprs| exprs.into_iter().map(
                        |expr| self.eval(expr).to_string()
                    ).collect::<Vec<String>>().concat()
                ).collect::<Vec<String>>();

                let mut cmd = Command::new(segments.remove(0));
                cmd.args(segments);

                cmd.stdin(stdin);
                cmd.stdout(stdout);
                cmd.stderr(stderr);

                let child = cmd.spawn().unwrap();

                drop(cmd);

                Process::Std(child)
            }
            Cmd::Op(lhs, CmdOp::OutPipe, rhs) => {
                let (r, w) = pipe().unwrap();

                let lhs = self.run_cmd(*lhs, stdin, Stream::PipeWriter(w), Stream::Null);
                let rhs = self.run_cmd(*rhs, Stream::PipeReader(r), stdout, stderr);

                Process::Pipe {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }
            }
            Cmd::Op(lhs, CmdOp::And, rhs) => {
                let (in_1, in_2) = (stdin.clone(), stdin);
                let (out_1, out_2) = (stdout.clone(), stdout);
                let (err_1, err_2) = (stderr.clone(), stderr);

                let mut lhs = self.run_cmd(*lhs, in_1, out_1, err_1);

                let t = thread::spawn(|| {
                    let exit_code = lhs.wait();
                    if exit_code.success() {
                        let mut rhs = self.run_cmd(*rhs, in_2, out_2, err_2);
                        rhs.wait()
                    } else {
                        exit_code
                    }
                });

                Process::And {
                    lhs: Box::new(lhs),
                    t,
                }
            }
            _ => todo!()
        }
    }

    fn eval(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Literal(value) => value,
            Expr::Vec(vec) => {
                let vec = vec.into_iter().map(|expr| self.eval(expr)).collect::<Vec<Value>>();
                Value::Vec(vec)
            }
            Expr::Dict(dict) => {
                let dict = dict.into_iter().map(|(key, expr)| (key, self.eval(expr))).collect::<HashMap<String, Value>>();
                Value::Dict(dict)
            }
            _ => todo!()
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Func {
    User {
        name: Option<String>,
        params: Vec<String>,
        body: Box<Stmt>,
    },
    Native {
        name: String,
        body: fn(Vec<Value>) -> Value,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Nil,
    Num(f64),
    String(String),
    Bool(bool),

    Vec(Vec<Value>),
    Dict(HashMap<String, Value>),

    Func(Func),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Num(num) => write!(f, "{}", num),
            Value::String(string) => write!(f, "{}", string),
            Value::Bool(bool) => write!(f, "{}", bool),
            Value::Vec(vec) => {
                write!(f, "[{}]", vec.iter().map(|v| v.to_string_quoted()).join(", "))
            }
            Value::Dict(dict) => {
                write!(f, "{{{}}}", dict.iter().map(|(k, v)| format!("{}: {}", k, v.to_string_quoted())).join(", "))
            }
            Value::Func(func) => match func {
                Func::User { name, .. } => match name {
                    Some(name) => write!(f, "<func {}>", name),
                    None => write!(f, "<lambda func>"),
                },
                Func::Native { name, .. } => write!(f, "<native func {}>", name),
            },
        }
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(false) => false,
            _ => true,
        }
    }

    pub fn to_string_quoted(&self) -> String {
        if !matches!(self, Value::String(..)) {
            self.to_string()
        } else {
            format!("\'{}\'", self.to_string())
        }
    }
}
