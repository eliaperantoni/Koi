use crate::ast::{Cmd, Expr, Stmt, Value};
use crate::token::{Token, TokenKind};

use super::Parser;

impl Parser {
    pub fn parse_cmd(&mut self) -> Cmd {
        let tokens: Vec<Token> = self.lexer.by_ref().take_while(
            |t| !matches!(t, Token{kind: TokenKind::Newline, ..}),
        ).collect();

        Vec::new()
    }
}
