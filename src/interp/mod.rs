use core::fmt;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::panic::panic_any;

use itertools::Itertools;

use crate::ast::{BinaryOp, Expr, Prog, Stmt, UnaryOp};
use crate::ast::Expr::Interp;
use crate::interp::stack::Stack;

mod cmd;

mod stack;

#[cfg(test)]
mod test;

pub struct Interpreter {
    stack: Stack,
    collector: Option<String>,
}

fn print(int: &mut Interpreter, args: Vec<Value>) -> Value {
    for val in args {
        if let Some(str) = &mut int.collector {
            str.push_str(&mut (val.to_string() + "\n"));
        } else {
            println!("{}", val);
        }
    }
    Value::Nil
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut interpreter = Interpreter {
            stack: Stack::new(),
            collector: None,
        };
        interpreter.init_native_funcs();
        interpreter
    }

    pub fn run(&mut self, prog: Prog) {
        for stmt in prog.into_iter() {
            self.run_stmt(stmt);
        }
    }

    pub fn do_collect(&mut self) {
        self.collector = Some(String::new());
    }

    fn init_native_funcs(&mut self) {
        self.stack.def("print".to_string(), Value::Func(Func::Native {
            name: "print".to_string(),
            func: print,
        }));
    }

    fn run_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Cmd(cmd) => {
                if self.collector.is_some() {
                    let output = self.run_cmd_capture(cmd);
                    self.collector.as_mut().unwrap().push_str(&output);
                } else {
                    self.run_cmd_pipe(cmd);
                }
            }
            Stmt::Let { name, init, .. } => {
                let value = match init {
                    Some(expr) => self.eval(expr),
                    _ => Value::Nil,
                };

                self.stack.def(name, value);
            }
            Stmt::Expr(expr) => {
                self.eval(expr);
            }
            Stmt::Block(stmts) => {
                self.stack.push();
                for stmt in stmts {
                    self.run_stmt(stmt);
                }
                self.stack.pop();
            }
            Stmt::For { lvar, rvar, iterated, each_do } => {
                let iterated = self.eval(iterated);

                match iterated {
                    Value::Range(l, r) => {
                        assert!(rvar.is_none(), "for loop with range does not need a second variable");

                        self.stack.push();
                        self.stack.def(lvar.clone(), Value::Num(l as f64));
                        for i in l..r {
                            *self.stack.get_mut(&lvar) = Value::Num(i as f64);
                            self.run_stmt(*each_do.clone());
                        }
                        self.stack.pop();
                    }
                    _ => todo!()
                }
            }
            Stmt::While { cond, then_do } => {
                self.stack.push();
                while self.eval(cond.clone()).is_truthy() {
                    self.run_stmt(*then_do.clone());
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
            Expr::Get(name) => self.stack.get(&name).clone(),
            Expr::Set(name, expr) => {
                let value = self.eval(*expr);
                *self.stack.get_mut(&name) = value.clone();
                value
            }
            Expr::Interp { mut strings, exprs } => {
                let mut out = String::new();

                out += &strings.remove(0);

                for expr in exprs {
                    let str = self.eval(expr).to_string();
                    out += &str;
                    out += &strings.remove(0);
                }

                Value::String(out)
            }
            Expr::Range { l, r, inclusive } => {
                let l = self.eval(*l);
                let r = self.eval(*r);

                match (l, r) {
                    // The x.trunc() == x part is to check that the numbers are integers
                    (Value::Num(l), Value::Num(r)) if l.trunc() == l && r.trunc() == r => {
                        Value::Range(l as i32, r as i32 + if inclusive { 1 } else { 0 })
                    }
                    _ => panic!("range must evaluate to integers")
                }
            }
            Expr::Binary(lhs, BinaryOp::Sum, rhs) => {
                match (self.eval(*lhs), self.eval(*rhs)) {
                    (Value::Num(lhs), Value::Num(rhs)) => Value::Num(lhs + rhs),
                    (Value::String(lhs), Value::String(rhs)) => Value::String(lhs + &rhs),
                    _ => panic!("invalid operands types for op {:?}", BinaryOp::Sum),
                }
            }
            Expr::Binary(lhs, op, rhs) if [
                BinaryOp::Sub, BinaryOp::Mul, BinaryOp::Div,
                BinaryOp::Mod, BinaryOp::Pow, BinaryOp::Less, BinaryOp::Great
            ].contains(&op) => {
                let (lhs, rhs) = match (self.eval(*lhs), self.eval(*rhs)) {
                    (Value::Num(lhs), Value::Num(rhs)) => (lhs, rhs),
                    _ => panic!("invalid operands types for op {:?}", op),
                };

                match op {
                    BinaryOp::Sub => Value::Num(lhs - rhs),
                    BinaryOp::Mul => Value::Num(lhs * rhs),
                    BinaryOp::Div => Value::Num(lhs / rhs),
                    BinaryOp::Mod => Value::Num(lhs % rhs),
                    BinaryOp::Pow => Value::Num(lhs.powf(rhs)),
                    BinaryOp::Less => Value::Bool(lhs < rhs),
                    BinaryOp::Great => Value::Bool(lhs > rhs),
                    _ => unreachable!(),
                }
            }
            Expr::Binary(lhs, BinaryOp::And, rhs) => {
                let lhs = self.eval(*lhs);
                if lhs.is_truthy() {
                    self.eval(*rhs)
                } else {
                    lhs
                }
            }
            Expr::Binary(lhs, BinaryOp::Or, rhs) => {
                let lhs = self.eval(*lhs);
                if lhs.is_truthy() {
                    lhs
                } else {
                    self.eval(*rhs)
                }
            }
            Expr::Binary(lhs, BinaryOp::Equal, rhs) => Value::Bool(self.eval(*lhs) == self.eval(*rhs)),
            Expr::Unary(UnaryOp::Not, expr) => Value::Bool(!self.eval(*expr).is_truthy()),
            Expr::Unary(UnaryOp::Neg, expr) => {
                let num = if let Value::Num(num) = self.eval(*expr) {
                    num
                } else {
                    panic!("invalid operand type for op {:?}", UnaryOp::Neg);
                };

                Value::Num(-num)
            }
            Expr::Comma(mut exprs) => {
                let last = exprs.remove(exprs.len() - 1);
                for expr in exprs {
                    self.eval(expr);
                }
                self.eval(last)
            }
            Expr::Call { func, args } => {
                let func = self.eval(*func);

                let args = args.into_iter().map(|expr| self.eval(expr)).collect();

                match func {
                    Value::Func(Func::Native { func, .. }) => {
                        func(self, args)
                    }
                    _ => panic!("attempt to call non-function")
                }
            }
            _ => todo!()
        }
    }
}

#[derive(Clone)]
pub enum Func {
    User {
        name: Option<String>,
        params: Vec<String>,
        body: Box<Stmt>,
    },
    Native {
        name: String,
        func: fn(&mut Interpreter, Vec<Value>) -> Value,
    },
}

impl Debug for Func {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Func::User { name, .. } => match name {
                Some(name) => write!(f, "<func {}>", name),
                None => write!(f, "<lambda func>"),
            },
            Func::Native { name, .. } => write!(f, "<native func {}>", name),
        }
    }
}

impl PartialEq for Func {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Func::User { name, .. }, Func::User { name: name_other, .. }) => name == name_other,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Nil,
    Num(f64),
    String(String),
    Bool(bool),

    Vec(Vec<Value>),
    Dict(HashMap<String, Value>),

    Range(i32, i32),

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
            Value::Func(func) => write!(f, "{:?}", func),
            Value::Range(l, r) => write!(f, "{}..{}", l, r),
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
