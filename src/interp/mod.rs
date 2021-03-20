use core::fmt;
use std::borrow::Borrow;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::env as std_env;
use std::fmt::{Debug, Display, Formatter};
use std::hint::unreachable_unchecked;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::panic::panic_any;
use std::rc::Rc;

use itertools::Itertools;

use crate::ast::{BinaryOp, Expr, Prog, Stmt, UnaryOp};
use crate::ast::Expr::Interp;
use crate::interp::env::{Env, Var};

mod cmd;
mod env;

#[cfg(test)]
mod test;

pub struct Interpreter {
    env: Rc<RefCell<Env>>,
    collector: Option<String>,
}

fn print(int: &mut Interpreter, args: Vec<Value>) -> Value {
    let mut res = args.iter().map(|arg| arg.to_string()).join(" ");

    if let Some(str) = &mut int.collector {
        str.push_str(&res);
        str.push_str("\n");
    } else {
        println!("{}", res);
    }

    Value::Nil
}

#[derive(Debug)]
enum Escape {
    Break,
    Continue,
    Return(Value)
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut interpreter = Interpreter {
            env: Rc::new(RefCell::new(Env::new())),
            collector: None,
        };
        interpreter.init_native_funcs();
        interpreter.import_os_env();
        interpreter.push_env();
        interpreter
    }

    pub fn run(&mut self, prog: Prog) {
        for stmt in prog.into_iter() {
            self.run_stmt(stmt).expect("escape bubbled up to top level");
        }
    }

    pub fn do_collect(&mut self) {
        self.collector = Some(String::new());
    }

    fn init_native_funcs(&mut self) {
        self.get_env_mut().def("print".to_string(), Value::Func(Func::Native {
            name: "print".to_string(),
            func: print,
        }));
    }

    fn import_os_env(&mut self) {
        for (k, v) in std_env::vars() {
            RefCell::borrow_mut(&self.env).def(k, Value::String(v));
        }
    }

    fn push_env(&mut self) {
        let new_env = Rc::new(RefCell::new(Env::new_from(&self.env)));
        mem::replace(&mut self.env, new_env);
    }

    fn pop_env(&mut self) {
        let parent_env = self.get_env().parent_ref();
        mem::replace(&mut self.env, parent_env);
    }

    fn get_env(&self) -> Ref<Env> {
        RefCell::borrow(&self.env)
    }

    fn get_env_mut(&mut self) -> RefMut<Env> {
        RefCell::borrow_mut(&self.env)
    }

    fn run_stmt(&mut self, stmt: Stmt) -> Result<(), Escape> {
        match stmt {
            Stmt::Cmd(cmd) => {
                let env = self.get_env().os_env();

                if self.collector.is_some() {
                    let output = self.run_cmd_capture(cmd, env);
                    self.collector.as_mut().unwrap().push_str(&output);
                } else {
                    self.run_cmd_pipe(cmd, env);
                }
            }
            Stmt::Let { name, init, is_exp } => {
                let val = match init {
                    Some(expr) => self.eval(expr),
                    _ => Value::Nil,
                };

                self.get_env_mut().def(name, Var::new(val, is_exp));
            }
            Stmt::Expr(expr) => {
                self.eval(expr);
            }
            Stmt::Block(stmts) => {
                self.push_env();
                for stmt in stmts {
                    self.run_stmt(stmt).or_else(|err| {
                        self.pop_env();
                        Err(err)
                    })?;
                }
                self.pop_env();
            }
            Stmt::For { lvar, rvar, iterated, each_do } => {
                let iterated = self.eval(iterated);

                match iterated {
                    Value::Range(l, r) => {
                        assert!(rvar.is_none(), "for loop with range does not need a second variable");

                        self.push_env();
                        self.get_env_mut().def(lvar.clone(), Value::Num(l as f64));

                        for i in l..r {
                            self.get_env_mut().put(&lvar, Value::Num(i as f64));

                            let res = self.run_stmt(*each_do.clone());
                            match &res {
                                Err(Escape::Continue) => continue,
                                Err(Escape::Break) => break,
                                _ => res?,
                            };
                        }

                        self.pop_env();
                    }
                    Value::Vec(vec) => {
                        let rvar = rvar.expect("for loop with vec does need a second variable");

                        self.push_env();
                        self.get_env_mut().def(lvar.clone(), Value::Nil);
                        self.get_env_mut().def(rvar.clone(), Value::Nil);

                        for (i, v) in RefCell::borrow(&vec).iter().enumerate() {
                            self.get_env_mut().put(&lvar, Value::Num(i as f64));
                            self.get_env_mut().put(&rvar, v.clone());

                            let res = self.run_stmt(*each_do.clone());
                            match &res {
                                Err(Escape::Continue) => continue,
                                Err(Escape::Break) => break,
                                _ => res?,
                            };
                        }

                        self.pop_env();
                    }
                    Value::Dict(dict) => {
                        let rvar = rvar.expect("for loop with vec does need a second variable");

                        self.push_env();
                        self.get_env_mut().def(lvar.clone(), Value::Nil);
                        self.get_env_mut().def(rvar.clone(), Value::Nil);

                        for (k, v) in RefCell::borrow(&dict).iter() {
                            self.get_env_mut().put(&lvar, Value::String(k.clone()));
                            self.get_env_mut().put(&rvar, v.clone());

                            let res = self.run_stmt(*each_do.clone());
                            match &res {
                                Err(Escape::Continue) => continue,
                                Err(Escape::Break) => break,
                                _ => res?,
                            };
                        }

                        self.pop_env();
                    }
                    _ => unreachable!()
                }
            }
            Stmt::While { cond, then_do } => {
                while self.eval(cond.clone()).is_truthy() {
                    let res = self.run_stmt(*then_do.clone());
                    match &res {
                        Err(Escape::Continue) => continue,
                        Err(Escape::Break) => break,
                        _ => res?,
                    };
                }
            }
            Stmt::If { cond, then_do, else_do } => {
                if self.eval(cond).is_truthy() {
                    self.run_stmt(*then_do)?;
                } else if else_do.is_some() {
                    self.run_stmt(*else_do.unwrap())?;
                }
            }
            Stmt::Continue => return Err(Escape::Continue),
            Stmt::Break => return Err(Escape::Break),
            Stmt::Func(func) => {
                match &func {
                    // Lambdas don't get parsed as Stmt::Func but Expr::Lambda, therefore a name should always be present
                    Func::User { name, .. } => {
                        let name = name.as_ref().unwrap().clone();
                        self.get_env_mut().def(name, Value::Func(func));
                    }
                    Func::Native { .. } => unreachable!(),
                }
            }
            Stmt::Return(expr) => {
                let ret_val = if let Some(expr) = expr {
                    self.eval(expr)
                } else {
                    Value::Nil
                };

                return Err(Escape::Return(ret_val));
            }
        };
        Ok(())
    }

    fn eval(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Literal(value) => value,
            Expr::Vec(vec) => {
                let vec = vec.into_iter().map(|expr| self.eval(expr)).collect::<Vec<Value>>();
                let vec = Rc::new(RefCell::new(vec));
                Value::Vec(vec)
            }
            Expr::Dict(dict) => {
                let dict = dict.into_iter().map(|(key, expr)| (key, self.eval(expr))).collect::<HashMap<String, Value>>();
                let dict = Rc::new(RefCell::new(dict));
                Value::Dict(dict)
            }
            Expr::Cmd(cmd) => {
                let os_env = self.get_env().os_env();
                Value::String(self.run_cmd_capture(cmd, os_env))
            },
            Expr::Get(name) => RefCell::borrow(&self.env).get(&name).clone(),
            Expr::GetField {base, index} => {
                let base = self.eval(*base);
                let index = self.eval(*index);

                match base {
                    Value::Vec(vec) => {
                        let index = match index {
                            Value::Num(num) if num.trunc() == num => num as usize,
                            _ => panic!("bad index, want integer"),
                        };

                        RefCell::borrow(&vec)[index].clone()
                    }
                    Value::Dict(dict) => {
                        let index = match index {
                            Value::String(str) => str,
                            _ => panic!("bad index, want string"),
                        };

                        RefCell::borrow(&dict).get(&index).cloned().unwrap()
                    },
                    _ => panic!("bad get target"),
                }
            }
            Expr::Set(name, expr) => {
                let value = self.eval(*expr);
                self.get_env_mut().put(&name, value.clone());
                value
            }
            Expr::SetField { base, index, expr } => {
                let base = self.eval(*base);
                let index = self.eval(*index);
                let value = self.eval(*expr);

                match base {
                    Value::Vec(vec) => {
                        let index = match index {
                            Value::Num(num) if num.trunc() == num => num as usize,
                            _ => panic!("bad index, want integer"),
                        };

                        vec.borrow_mut()[index] = value.clone();
                    }
                    Value::Dict(dict) => {
                        let index = match index {
                            Value::String(str) => str,
                            _ => panic!("bad index, want string"),
                        };

                        dict.borrow_mut().insert(index, value.clone());
                    },
                    _ => panic!("bad assignment target"),
                };

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
            Expr::Call { func, args } => {
                let func = self.eval(*func);
                let func = match func {
                    Value::Func(func) => func,
                    _ => panic!("attempt to call non-function"),
                };

                let args: Vec<Value> = args.into_iter().map(|expr| self.eval(expr)).collect();

                match func {
                    Func::User { params, body, .. } => {
                        assert_eq!(params.len(), args.len(), "number of arguments does not match number of parameters");

                        let mut func_env = Rc::new(RefCell::new(Env::new()));
                        let mut callee_env = mem::replace(&mut self.env, func_env);

                        for (param, arg) in params.into_iter().zip(args.into_iter()) {
                            self.get_env_mut().def(param, arg);
                        }

                        let res = self.run_stmt(*body);

                        mem::swap(&mut self.env, &mut callee_env);

                        match res {
                            Err(Escape::Return(val)) => val,
                            Err(err) => panic!("non return escape outside function"),
                            _ => Value::Nil,
                        }
                    }
                    Func::Native { func, .. } => {
                        func(self, args)
                    }
                }
            }
            Expr::Lambda(func) => Value::Func(func),
            _ => unreachable!()
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

    Vec(Rc<RefCell<Vec<Value>>>),
    Dict(Rc<RefCell<HashMap<String, Value>>>),

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
                let vec = RefCell::borrow(vec);
                write!(f, "[{}]", vec.iter().map(|v| v.to_string_quoted()).join(", "))
            }
            Value::Dict(dict) => {
                let dict = RefCell::borrow(dict);
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
