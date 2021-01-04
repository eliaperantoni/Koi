use crate::ast::{Value, Expr, Stmt};
use crate::token::{Token, TokenKind};

use super::Parser;

impl Parser {
    pub fn parse_stmt(&mut self) -> Stmt {
        Stmt::Expr(Expr::Literal(Value::Nil))
    }
}
