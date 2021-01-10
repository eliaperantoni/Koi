use super::Parser;

use crate::interp::Func;
use crate::token::{Token, TokenKind};

impl Parser {
    pub fn continue_parse_fn(&mut self) -> Func {
        let mut params = Vec::new();

        if !matches!(self.lexer.next(), Some(Token{kind: TokenKind::LeftParen, ..})) {
            panic!("expected left parenthesis");
        }

        self.lexer.consume_whitespace(self.is_multiline);
        if matches!(self.lexer.peek(), Some(Token { kind: TokenKind::RightParen, .. })) {
            self.lexer.next();
        } else {
            loop {
                self.lexer.consume_whitespace(self.is_multiline);
                params.push(self.must_identifier());
                self.lexer.consume_whitespace(self.is_multiline);

                match self.lexer.next() {
                    Some(Token { kind: TokenKind::Comma, .. }) => (),
                    Some(Token { kind: TokenKind::RightParen, .. }) => break,
                    _ => panic!("expected comma or right parenthesis"),
                }
            }
        }

        self.lexer.consume_whitespace(self.is_multiline);
        let body = self.parse_block();

        Func {
            name: None,
            params,
            body: Box::new(body),
        }
    }
}