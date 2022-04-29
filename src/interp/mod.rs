use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::env as std_env;
use std::fmt::Debug;
use std::mem;
use std::rc::Rc;
use std::path::PathBuf;

pub use func::Func;
pub use value::Value;

use crate::ast::{BinaryOp, Expr, Prog, Stmt, UnaryOp};
use crate::interp::env::{Env, Var};
use crate::lexer::new as new_lexer;
use crate::parser;

mod cmd;
mod env;
mod value;
mod func;
mod native;
mod method;

#[cfg(test)]
mod test;

pub struct Interpreter {
    env: Rc<RefCell<Env>>,
    collector: Option<String>,
    import_root: PathBuf,
}

#[derive(Debug)]
enum Escape {
    Break,
    Continue,
    Return(Value),
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut interpreter = Interpreter {
            env: Rc::new(RefCell::new(Env::new())),
            collector: None,
            import_root: std::env::current_dir().expect("couldn't get working dir"),
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
        use native::*;

        self.get_env_mut().def("print".to_string(), Value::Func(Func::Native {
            name: "print".to_string(),
            params: None,
            func: print,
            receiver: None,
        }));

        self.get_env_mut().def("input".to_string(), Value::Func(Func::Native {
            name: "input".to_string(),
            params: None,
            func: input,
            receiver: None,
        }));

        self.get_env_mut().def("exit".to_string(), Value::Func(Func::Native {
            name: "exit".to_string(),
            params: Some(1),
            func: exit,
            receiver: None,
        }));

        self.get_env_mut().def("glob".to_string(), Value::Func(Func::Native {
            name: "glob".to_string(),
            params: Some(1),
            func: glob,
            receiver: None,
        }));
    }

    pub fn set_import_root(&mut self, import_root: PathBuf) {
        self.import_root = import_root;
    }

    fn import_os_env(&mut self) {
        for (k, v) in std_env::vars() {
            RefCell::borrow_mut(&self.env).def(k, Value::String(v));
        }
    }

    fn push_env(&mut self) {
        self.env = Rc::new(RefCell::new(Env::new_from(&self.env)));
    }

    fn pop_env(&mut self) {
        let parent_env = self.get_env().parent_ref();
        self.env = parent_env;
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
                    let output = self.run_cmd_capture(cmd, env, true);
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

                self.push_env();
                self.get_env_mut().def(name, Var::new(val, is_exp));
            }
            Stmt::Import(mut base_path) => {
                if !base_path.ends_with(".koi") {
                    base_path.push_str(".koi");
                }

                let mut path = self.import_root.clone();
                path.push(&base_path);

                let source = std::fs::read_to_string(&path)
                    .expect("reading imported file's content");
                let prog = parser::Parser::new(new_lexer(source)).parse();

                path.pop();

                let old_import_root = std::mem::replace(&mut self.import_root, path);
                self.run(prog);
                self.import_root = old_import_root;
            }
            Stmt::Expr(expr) => {
                self.eval(expr);
            }
            Stmt::Block(stmts) => {
                let original_env = Rc::clone(&self.env);

                self.push_env();

                let mut res = Ok(());
                for stmt in stmts {
                    res = self.run_stmt(stmt);
                    if res.is_err() {
                        break;
                    }
                }

                while !Rc::ptr_eq(&self.env, &original_env) {
                    self.pop_env();
                }

                res?
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
                match func {
                    // Lambdas don't get parsed as Stmt::Func but Expr::Lambda, therefore a name should always be present
                    Func::User { name, params, body, .. } => {
                        let func = Value::Func(Func::User {
                            name: name.clone(),
                            params,
                            body,
                            captured_env: Some(Rc::clone(&self.env)),
                        });
                        self.get_env_mut().def(name.unwrap(), func);
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
                Value::String(self.run_cmd_capture(cmd, os_env, false))
            }
            Expr::Get(name) => RefCell::borrow(&self.env).get(&name).clone(),
            Expr::GetField { base, index } => {
                let base = self.eval(*base);
                let index = self.eval(*index);

                let val = match (base.clone(), index.clone()) {
                    (Value::Vec(vec), Value::Range(from, to)) => {
                        let vec = RefCell::borrow(&vec);
                        let vec = &vec[from..to];
                        Some(Value::Vec(Rc::new(RefCell::new(vec.to_vec()))))
                    }
                    (Value::Vec(vec), Value::Num(index)) => {
                        let index = if index.trunc() == index {
                            index as usize
                        } else {
                            panic!("expected integer index")
                        };

                        RefCell::borrow(&vec).get(index).cloned()
                    }
                    (Value::Dict(dict), index @ Value::String(_) | index @ Value::Num(_)) => {
                        let index = dict_key(index);

                        RefCell::borrow(&dict).get(&index).cloned()
                    }
                    _ => None
                };

                if let Some(val) = val {
                    val
                } else {
                    let method_name = if let Value::String(method_name) = index {
                        method_name
                    } else {
                        panic!("expected string index");
                    };

                    self.build_native_method(base, method_name)
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

                match (base, index) {
                    (Value::Vec(vec), Value::Num(index)) => {
                        let index = if index.trunc() == index {
                            index as usize
                        } else {
                            panic!("expected integer index")
                        };

                        vec.borrow_mut()[index] = value.clone();
                    }
                    (Value::Dict(dict), index @ Value::String(_) | index @ Value::Num(_)) => {
                        let index = dict_key(index);

                        dict.borrow_mut().insert(index, value.clone());
                    }
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
                        Value::Range(l as usize, r as usize + if inclusive { 1 } else { 0 })
                    }
                    _ => panic!("range must evaluate to integers")
                }
            }
            Expr::Binary(lhs, BinaryOp::Sum, rhs) => {
                match (self.eval(*lhs), self.eval(*rhs)) {
                    (Value::Num(lhs), Value::Num(rhs)) => Value::Num(lhs + rhs),
                    (Value::String(lhs), Value::String(rhs)) => Value::String(lhs + &rhs),
                    (Value::Vec(lhs), Value::Vec(rhs)) => {
                        let lhs = RefCell::borrow(&lhs);
                        let rhs = RefCell::borrow(&rhs);

                        let mut res = Vec::new();
                        res.append(&mut lhs.clone());
                        res.append(&mut rhs.clone());

                        Value::Vec(Rc::new(RefCell::new(res)))
                    }
                    (Value::Dict(lhs), Value::Dict(rhs)) => {
                        let lhs = RefCell::borrow(&lhs);
                        let rhs = RefCell::borrow(&rhs);

                        let mut res = HashMap::new();
                        for (k, v) in lhs.clone().into_iter() {
                            res.insert(k, v);
                        }
                        for (k, v) in rhs.clone().into_iter() {
                            res.insert(k, v);
                        }

                        Value::Dict(Rc::new(RefCell::new(res)))
                    }
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
                let args: Vec<Value> = args.into_iter().map(|expr| self.eval(expr)).collect();

                self.call(func, args)
            }
            Expr::Lambda(func) => match func {
                Func::User { name, params, body, .. } => Value::Func(Func::User {
                    name,
                    params,
                    body,
                    captured_env: Some(Rc::clone(&self.env)),
                }),
                Func::Native { .. } => unreachable!()
            }
            _ => unreachable!()
        }
    }

    fn call(&mut self, func: Value, mut args: Vec<Value>) -> Value {
        let func = match func {
            Value::Func(func) => func,
            _ => panic!("attempt to call non-function"),
        };

        match func {
            Func::User { params, body, captured_env, .. } => {
                assert_eq!(params.len(), args.len(), "number of arguments does not match number of parameters");

                let func_env = Rc::new(RefCell::new(if let Some(captured_env) = captured_env {
                    Env::new_from(&captured_env)
                } else {
                    Env::new()
                }));

                let mut callee_env = mem::replace(&mut self.env, func_env);

                for (param, arg) in params.into_iter().zip(args.into_iter()) {
                    self.get_env_mut().def(param, arg);
                }

                let res = self.run_stmt(*body);

                mem::swap(&mut self.env, &mut callee_env);

                match res {
                    Err(Escape::Return(val)) => val,
                    Err(_) => panic!("non return escape outside function"),
                    _ => Value::Nil,
                }
            }
            Func::Native { func, params, receiver, .. } => {
                if let Some(receiver) = receiver {
                    args.insert(0, *receiver);
                }

                if let Some(params) = params {
                    assert_eq!(params, args.len(), "number of arguments does not match number of parameters");
                }

                func(self, args)
            }
        }
    }

    pub fn set_args(&mut self, args: Vec<String>) {
        self.get_env_mut().def("args".to_string(), Value::Vec(Rc::new(RefCell::new(
            args.into_iter().map(|s| Value::String(s)).collect()
        ))));
    }
}

fn dict_key(val: Value) -> String {
    match val {
        Value::String(str) => str,
        Value::Num(num) => num.to_string(),
        _ => panic!("expected num or string"),
    }
}
