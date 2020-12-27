use crate::ast::{Expr, Stmt};
use crate::token::{Token, TokenKind};

use super::Parser;

impl Parser {
    pub fn parse_stmt(&mut self) -> Stmt {
        let was_line_start = self.lexer.is_line_start();

        if let Ok(stmt) = self.parse_expr_stmt() {
            stmt
        } else if was_line_start {
            self.lexer.rewind_line();
            Stmt::Cmd(self.parse_cmd())
        } else {
            panic!("bad statement");
        }
    }

    fn parse_expr_stmt(&mut self) -> Result<Stmt, &'static str> {
        let expr = self.parse_expression(0)?;

        if !matches!(expr, Expr::Set(..) | Expr::SetField {..} | Expr::Call {..}) {
            return Err("expression is neither assignment nor call");
        }

        Ok(Stmt::Expr(expr))
    }
}
