use itertools::__std_iter::Peekable;

use crate::lexer::Lexer;
use crate::ast::Stmt;
use crate::token::{Token, TokenKind};

mod expr;
mod stmt;
mod cmd;

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

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();

        loop {
            self.lexer.consume_whitespace(self.is_multiline);
            if self.is_at_end() {
                break;
            }

            stmts.push(self.parse_stmt());
        }

        stmts
    }

    pub fn is_at_end(&mut self) -> bool {
        match self.lexer.peek() {
            Some(Token{kind: TokenKind::Newline, ..}) if !self.is_multiline => true,
            None => true,
            _ => false,
        }
    }
}
