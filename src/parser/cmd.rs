use crate::ast::{Cmd, Expr, Stmt, Value};
use crate::token::{Token, TokenKind};

use super::Parser;

impl Parser {
    pub fn parse_cmd(&mut self) -> Cmd {
        let tokens: Vec<Token> = self.lexer.by_ref().take_while(
            |t| !matches!(t, Token{kind: TokenKind::Newline, ..}),
        ).collect();

        let string: String = tokens.into_iter().map(|t|t.lexeme).collect::<Vec<String>>().concat();

        vec![vec![Expr::Literal(Value::String(string))]]
    }
}
