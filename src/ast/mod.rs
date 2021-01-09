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
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    Num(f64),
    String(String),
    Bool(bool),

    Vec(Vec<Value>),
    Dict(HashMap<Value, Value>),

    Func {
        args: Vec<String>,
        stmts: Vec<Stmt>,
    },
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Nil, Nil) => true,
            (Num(this), Num(other)) => this == other,
            (String(this), String(other)) => this == other,
            (Bool(this), Bool(other)) => this == other,
            (Vec(this), Vec(other)) => false,
            (Dict(this), Dict(other)) => false,
            (
                Func { args: this_args, stmts: this_stmts },
                Func { args: other_args, stmts: other_stmts },
            ) => this_args == other_args && this_stmts == other_stmts,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CmdOp {
    And,
    Or,

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
