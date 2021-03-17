use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use itertools::Itertools;

use crate::ast::{Expr, Prog, Stmt};
use crate::interp::stack::Stack;

mod cmd;

mod stack;

#[cfg(test)]
mod test;

pub struct Interpreter {
    stack: Stack,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut interpreter = Interpreter {
            stack: Stack::new(),
        };
        interpreter.init_native_funcs();
        interpreter
    }

    pub fn run(&mut self, prog: Prog) {
        for stmt in prog.into_iter() {
            self.run_stmt(stmt);
        }
    }

    fn init_native_funcs(&mut self) {
        self.stack.globals().insert("print".to_string(), Value::Func(Func::Native {
            name: "print".to_string(),
            func: |args: Vec<Value>| -> Value {
                for val in args {
                    println!("{}", val);
                }
                Value::Nil
            },
        }));
    }

    fn run_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Cmd(cmd) => {
                self.run_cmd_pipe(cmd);
            }
            Stmt::Let { name, init, .. } => {
                let val = match init {
                    Some(expr) => self.eval(expr),
                    _ => Value::Nil,
                };

                self.stack.globals().insert(name, val);
            }
            Stmt::Expr(expr) => {
                match expr {
                    Expr::Cmd(cmd) => self.run_cmd_pipe(cmd),
                    Expr::Call { func, args } => {
                        let func = self.eval(*func);

                        let args = args.into_iter().map(|expr| self.eval(expr)).collect();

                        match func {
                            Value::Func(Func::Native { func, .. }) => {
                                func(args);
                            }
                            _ => panic!("attempt to call non-function")
                        }
                    }
                    Expr::Set(..) => todo!(),
                    Expr::SetField { .. } => todo!(),
                    _ => unreachable!()
                }
            }
            Stmt::Block(stmts) => {
                self.stack.push();
                for stmt in stmts {
                    self.run_stmt(stmt);
                }
                self.stack.pop();
            }
            _ => todo!(),
        };
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
            Expr::Cmd(cmd) => Value::String(self.run_cmd_capture(cmd)),
            Expr::Get(name) => self.stack.lookup(&name),
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
        func: fn(Vec<Value>) -> Value,
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
