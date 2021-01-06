use itertools::Itertools;

use crate::ast::{Expr, Stmt, Value};
use crate::token::{Token, TokenKind};

use super::Parser;

impl Parser {
    pub fn parse_stmt(&mut self) -> Stmt {
        match self.lexer.peek() {
            Some(Token {kind: TokenKind::Let, ..}) |
            Some(Token {kind: TokenKind::Exp, ..}) => self.parse_let_stmt(),

            Some(Token{kind: TokenKind::If, ..}) => self.parse_if_stmt(),
            Some(Token{kind: TokenKind::For, ..}) => self.parse_for_stmt(),
            Some(Token{kind: TokenKind::While, ..}) => self.parse_while_stmt(),
            Some(Token{kind: TokenKind::Fn, ..}) => self.parse_fn_stmt(),

            _ => {
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
    }

    fn parse_let_stmt(&mut self) -> Stmt {
        let is_exp = self.lexer.peek().unwrap().kind == TokenKind::Exp;

        if is_exp {
            self.lexer.next();
            self.lexer.consume_whitespace();
        }

        // Only meaningful if there was an `exp`. Otherwise this has already been checked by `parse_stmt`
        if !matches!(self.lexer.next(), Some(Token{kind: TokenKind::Let, ..})) {
            panic!("expected let");
        }

        self.lexer.consume_whitespace();

        let name = if let Some(Token {kind: TokenKind::Identifier(name), ..}) = self.lexer.next() {
            name
        } else {
            panic!("expected identifier");
        };

        self.lexer.consume_whitespace();

        if matches!(self.lexer.peek(), Some(Token{kind: TokenKind::Equal, ..})) {
            self.lexer.next();
            self.lexer.consume_whitespace();
            Stmt::Let {
                is_exp,
                name,
                init: Some(self.parse_expression(0)),
            }
        } else {
            Stmt::Let {
                is_exp,
                name,
                init: None,
            }
        }
    }

    fn parse_if_stmt(&mut self) -> Stmt {
        todo!()
    }

    fn parse_for_stmt(&mut self) -> Stmt {
        todo!()
    }

    fn parse_while_stmt(&mut self) -> Stmt {
        todo!()
    }

    fn parse_fn_stmt(&mut self) -> Stmt {
        todo!()
    }
}
