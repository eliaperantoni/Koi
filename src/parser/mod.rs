use crate::ast::{Prog, Stmt};
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

mod expr;
mod stmt;
mod cmd;
mod func;

#[cfg(test)]
mod test;

pub struct Parser {
    lexer: Lexer,
    is_multiline: bool,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer,
            is_multiline: true,
        }
    }

    pub fn parse(&mut self) -> Prog {
        self.parse_stmts()
    }

    fn parse_stmts(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();

        loop {
            self.lexer.consume_whitespace(self.is_multiline);

            if self.is_at_end() {
                break;
            }

            if self.lexer.peek().unwrap().kind == TokenKind::RightBrace {
                // The lexer should already catch the error where more blocks are closed than opened
                break;
            }

            stmts.push(self.parse_stmt());
        }

        stmts
    }

    pub fn is_at_end(&mut self) -> bool {
        match self.lexer.peek() {
            Some(Token { kind: TokenKind::Newline, .. }) if !self.is_multiline => true,
            None => true,
            _ => false,
        }
    }

    pub fn must_identifier(&mut self) -> String {
        if let Some(Token { kind: TokenKind::Identifier(name), .. }) = self.lexer.next() {
            name
        } else {
            panic!("expected identifier");
        }
    }
}
