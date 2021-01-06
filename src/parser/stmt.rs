use itertools::Itertools;

use crate::ast::{Expr, Stmt, Value};
use crate::token::{Token, TokenKind};

use super::Parser;

impl Parser {
    pub fn parse_stmt(&mut self) -> Stmt {
        if matches!(self.lexer.peek(), Some(Token {kind: TokenKind::Dollar, ..})) {
            self.lexer.next();
            Stmt::Cmd(self.parse_cmd(0))
        } else if !self.is_expr_next() {
            Stmt::Cmd(self.parse_cmd(0))
        } else {
            let expr = self.parse_expression(0);
            if !matches!(expr, Expr::Set(..) | Expr::SetField {..} | Expr::Call {..}) {
                panic!("only assignment and call expressions are allowed as statements");
            }
            Stmt::Expr(expr)
        }
    }

    fn is_expr_next(&mut self) -> bool {
        if !self.lexer.is_new_line {
            return true;
        }

        self.lexer.start_recording();

        let line_tokens = self.lexer
            .by_ref()
            .take_while(|t| t.kind != TokenKind::Newline)
            .filter(|t| ![TokenKind::Space, TokenKind::Newline].contains(&t.kind))
            .collect::<Vec<Token>>();

        self.lexer.stop_recording(true);

        let mut line_tokens_iter = line_tokens.iter().peekable();

        loop {
            if !matches!(line_tokens_iter.next(), Some(Token {kind: TokenKind::Identifier(..), ..})) {
                return false;
            }

            if matches!(line_tokens_iter.peek(), Some(&Token {kind: TokenKind::Dot, ..})) {
                line_tokens_iter.next();
                continue;
            }

            if matches!(line_tokens_iter.next(),
                Some(Token {kind: TokenKind::LeftParen, ..}) |
                Some(Token {kind: TokenKind::LeftBracket, ..}) |
                Some(Token {kind: TokenKind::Equal, ..})
            ) {
                return true;
            }
        }

        return false;
    }
}
