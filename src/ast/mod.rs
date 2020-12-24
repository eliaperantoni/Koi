use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BinaryOp {
    Sum,
    Sub,
    Mul,
    Div,
    Mod,
    Perc,
    And,
    Or,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Literal(Value),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Interp(Vec<String>, Vec<Expr>),

    Get(String),
    Set(String, Box<Expr>),

    GetVec(Box<Expr>, usize),
    SetVec(Box<Expr>, usize, Box<Expr>),

    GetDict(Box<Expr>, String),
    SetDict(Box<Expr>, String, Box<Expr>),
}

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    Num(f64),
    String(String),
    Bool(bool),
    Vec(Vec<Value>),
    Dict(HashMap<Value, Value>),
}
