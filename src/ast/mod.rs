use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum UnaryOp {
    Neg,
    Not,

    PreDec,
    PreInc,

    PostDec,
    PostInc,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BinaryOp {
    Sum,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,

    Great,
    Less,

    Equal,

    And,
    Or,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Literal(Value),

    Vec(Vec<Expr>),
    Dict(HashMap<String, Expr>),

    Interp {
        strings: Vec<String>,
        exprs: Vec<Expr>,
    },

    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),

    Get(String),
    Set(String, Box<Expr>),

    GetField {
        base: Box<Expr>,
        index: Box<Expr>,
    },
    SetField {
        base: Box<Expr>,
        index: Box<Expr>,
        value: Box<Expr>,
    },

    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },

    Cmd(Cmd),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Cmd(Cmd),
    Let {
        is_exp: bool,
        name: String,
        init: Option<Expr>,
    },
    Block(Vec<Stmt>),
    If {
        cond: Expr,
        then_do: Box<Stmt>,
        else_do: Option<Box<Stmt>>,
    },
    For {
        key_var: String,
        val_var: String,
        iterated: Expr,
        each_do: Box<Stmt>,
    },
    While {
        cond: Expr,
        then_do: Box<Stmt>,
    },
    Func(Func),
    Continue,
    Break,
    Return(Option<Expr>),
}

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
    Dict(HashMap<Value, Value>),

    Func(Func),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil,Value:: Nil) => true,
            (Value::Num(this), Value::Num(other)) => this == other,
            (Value::String(this), Value::String(other)) => this == other,
            (Value::Bool(this), Value::Bool(other)) => this == other,
            (Value::Vec(this), Value::Vec(other)) => false,
            (Value::Dict(this), Value::Dict(other)) => false,
            (Value::Func(
                Func {
                    params: this_params,
                    body: this_body, ..
                },
            ), Value::Func(
                Func {
                    params: other_params,
                    body: other_body, ..
                },
            )) => this_params == other_params && this_body == other_body,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CmdOp {
    And,
    Or,
    Seq,

    OutPipe,
    ErrPipe,
    AllPipe,

    OutWrite,
    ErrWrite,
    AllWrite,

    OutRead,
    ErrRead,
    AllRead,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Cmd {
    Atom(Vec<Vec<Expr>>),
    Op(Box<Cmd>, CmdOp, Box<Cmd>),
}
