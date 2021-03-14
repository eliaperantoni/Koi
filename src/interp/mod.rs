use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, ExitStatus, Stdio};
use std::rc::Rc;

use itertools::Itertools;
use os_pipe::{pipe, PipeWriter, PipeReader};

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
}

impl Process {
    fn wait(&mut self) -> ExitStatus {
        match self {
            Process::Std(child) => child.wait().unwrap(),
            Process::Pipe { lhs, rhs } => {
                lhs.wait();
                rhs.wait()
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
                let mut cmd = self.run_cmd::<Stdio,Stdio,Stdio>(
                    cmd,
                    None,
                    Some(Stdio::inherit()),
                    Some(Stdio::inherit()),
                );
                cmd.wait();
            }
            _ => todo!(),
        };
    }

    fn run_cmd<T, U, V>(&mut self, cmd: Cmd, stdin: Option<T>, stdout: Option<U>, stderr: Option<V>) -> Process
        where T: Into<Stdio>, U: Into<Stdio>, V: Into<Stdio> {
        match cmd {
            Cmd::Atom(segments) => {
                let mut segments = segments.into_iter().map(
                    |exprs| exprs.into_iter().map(
                        |expr| self.eval(expr).to_string()
                    ).collect::<Vec<String>>().concat()
                ).collect::<Vec<String>>();

                let mut cmd = Command::new(segments.remove(0));
                cmd.args(segments);

                if let Some(stdin) = stdin {
                    cmd.stdin(stdin);
                } else {
                    cmd.stdin(Stdio::null());
                }

                if let Some(stdout) = stdout {
                    cmd.stdout(stdout);
                } else {
                    cmd.stdout(Stdio::null());
                }

                if let Some(stderr) = stderr {
                    cmd.stderr(stderr);
                } else {
                    cmd.stderr(Stdio::null());
                }

                let child = cmd.spawn().unwrap();

                drop(cmd);

                Process::Std(child)
            }
            Cmd::Op(lhs, CmdOp::OutPipe, rhs) => {
                let (r, w) = pipe().unwrap();

                let lhs = self.run_cmd::<T, PipeWriter, V>(*lhs, stdin, Some(w), None);
                let rhs = self.run_cmd::<PipeReader, U, V>(*rhs, Some(r), stdout, stderr);

                Process::Pipe {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
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
