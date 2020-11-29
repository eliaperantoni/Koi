use crate::scanner::Token;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Num(f64),
    Bool(bool),
    String(String),
}

impl Value {
    pub fn stringify(&self) -> String {
        use Value::*;
        match self {
            String(string) => string.clone(),
            Num(num) => num.to_string(),
            Bool(bool) => bool.to_string(),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(true) => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Value(Value),
    Interp { segments: Vec<String>, exprs: Vec<Expr> },
    Unary { op: Token, rhs: Box<Expr> },
    Binary { lhs: Box<Expr>, rhs: Box<Expr>, op: Token },
}

impl From<Value> for Expr {
    fn from(value: Value) -> Self {
        Expr::Value(value)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Var {
        name: String,
        initializer: Option<Expr>,
    },
    If {
        cond: Expr,
        then_do: Vec<Stmt>,
        else_do: Vec<Stmt>,
    },
}

impl From<Expr> for Stmt {
    fn from(expr: Expr) -> Self {
        Stmt::Expr(expr)
    }
}
