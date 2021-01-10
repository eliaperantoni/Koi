use std::collections::HashMap;

use crate::ast::Stmt;

#[cfg(test)]
mod test;

#[derive(Clone, Debug, PartialEq)]
pub struct Func {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub body: Box<Stmt>,
}

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    Num(f64),
    String(String),
    Bool(bool),

    Vec(Vec<Value>),
    Dict(HashMap<String, Value>),

    Func(Func),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil,Value:: Nil) => true,

            (Value::Num(this), Value::Num(other)) => this == other,
            (Value::String(this), Value::String(other)) => this == other,
            (Value::Bool(this), Value::Bool(other)) => this == other,

            (Value::Vec(this), Value::Vec(other)) => this == other,
            (Value::Dict(this), Value::Dict(other)) => this == other,

            _ => false,
        }
    }
}
