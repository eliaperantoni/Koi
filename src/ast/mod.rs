use std::collections::HashMap;

pub enum UnaryOp {
    Neg,
    Not,
}

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

pub enum Value {
    Nil,
    Num(f64),
    String(String),
    Bool(bool),
    Vec(Vec<Value>),
    Dict(HashMap<Value, Value>),
}
