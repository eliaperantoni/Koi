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
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer,
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();

        loop {
            self.lexer.consume_whitespace();
            stmts.push(self.parse_stmt());

            self.lexer.consume_whitespace();
            if self.lexer.peek().is_none() {
                break;
            }
        }

        stmts
    }
}
