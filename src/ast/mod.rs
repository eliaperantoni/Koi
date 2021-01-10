use std::collections::HashMap;

use crate::interp::{Func, Value};

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

    Lambda(Func),
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
