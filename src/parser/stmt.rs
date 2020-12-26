use crate::ast::{Expr, Stmt};
use crate::token::{Token, TokenKind};

use super::Parser;

impl Parser {
    pub fn parse_stmt(&mut self) -> Stmt {
        if let Ok(stmt) = self.parse_expr_stmt() {
            stmt
        } else {
            Stmt::Cmd(self.parse_cmd())
        }
    }

    fn parse_expr_stmt(&mut self) -> Result<Stmt, &'static str> {
        let expr = self.parse_expression(0)?;

        if !matches!(expr, Expr::Set(..) | Expr::SetField {..} | Expr::Call {..}) {
            return Err("expression is neither assignment nor call");
        }

        if !matches!(self.lexer.peek(), Some(Token {kind: TokenKind::Newline, ..})) {
            return Err("no newline at end of expression statement");
        }

        self.lexer.next();

        Ok(Stmt::Expr(expr))
    }
}
