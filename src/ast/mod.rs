use crate::scanner::Token;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Int(i64),
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Value(Value),
    Un { op: Token, rhs: Box<Expr> },
    Bin { lhs: Box<Expr>, rhs: Box<Expr>, op: Token },
}

impl From<Value> for Expr {
    fn from(value: Value) -> Self {
        Expr::Value(value)
    }
}
