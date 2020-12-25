use crate::token::{Token, TokenKind};
use crate::ast::{Stmt, Expr};
use super::Parser;

impl Parser {
    pub fn parse_stmt(&mut self) -> Stmt {
        unimplemented!();
        /*
        let expr = self.parse_expression(0);
        match expr {
            Ok(Expr::Set(..)) | Ok(Expr::SetField {..}) => Stmt::Expr(expr),
            _ => {

            }
        }*/
    }
}
