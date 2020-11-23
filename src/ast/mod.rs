use crate::scanner::Token;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Int(i64),
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Value(Value),
    Unary { op: Token, rhs: Box<Expr> },
    Binary { lhs: Box<Expr>, rhs: Box<Expr>, op: Token },
    Paren { expr: Box<Expr> },
}

impl From<Value> for Expr {
    fn from(value: Value) -> Self {
        Expr::Value(value)
    }
}
