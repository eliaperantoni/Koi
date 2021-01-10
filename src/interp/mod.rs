use std::collections::HashMap;

use crate::ast::Stmt;
use std::ops::Deref;
use std::fmt::{Display, Formatter};
use itertools::Itertools;

#[cfg(test)]
mod test;

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
            },
            Value::Dict(dict) => {
                write!(f, "{{{}}}", dict.iter().map(|(k,v)| format!("{}: {}", k, v.to_string_quoted())).join(", "))
            },
            Value::Func(func) => match func {
                Func::User{name, ..} => match name {
                    Some(name) => write!(f, "<func {}>", name),
                    None => write!(f, "<lambda func>"),
                },
                Func::Native {name, ..} => write!(f, "<native func {}>", name),
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
