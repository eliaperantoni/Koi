use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::fmt;
use std::rc::Rc;

use crate::ast::Stmt;
use crate::interp::{Interpreter, Value};
use crate::interp::env::Env;

#[derive(Clone)]
pub enum Func {
    User {
        name: Option<String>,
        params: Vec<String>,
        body: Box<Stmt>,
        captured_env: Option<Rc<RefCell<Env>>>,
    },
    Native {
        name: String,
        params: Option<usize>,
        func: fn(&mut Interpreter, Vec<Value>) -> Value,
        receiver: Option<Box<Value>>,
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
