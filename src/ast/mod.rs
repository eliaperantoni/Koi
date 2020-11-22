use crate::scanner::Token;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Int(i64),
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Value(Value),
    Bin { lhs: Box<Expr>, rhs: Box<Expr>, op: Token },
}
