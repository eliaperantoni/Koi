use crate::ast::{Cmd, Expr, Value};
use crate::token::{Token, TokenKind};

use super::Parser;

impl Parser {
    pub fn parse_cmd(&mut self) -> Cmd {
        let mut lhs = {
            let mut segments = Vec::new();

            loop {
                let mut exprs = Vec::new();

                loop {
                    if self.lexer.peek().is_none() {
                        break;
                    }

                    if {
                        let t = self.lexer.peek().unwrap();
                        t.is_cmd_op() || [TokenKind::Space, TokenKind::Newline].contains(&t.kind)
                    } {
                        break;
                    }

                    let expr = match self.lexer.next().unwrap() {
                        t @ Token { kind: TokenKind::String { .. }, .. } => self.continue_parse_string_expr(t),
                        Token { kind: TokenKind::LeftBrace, .. } => {
                            self.lexer.consume_whitespace();
                            let expr = self.parse_expression(0);
                            self.lexer.consume_whitespace();

                            if !matches!(self.lexer.next(), Some(Token { kind: TokenKind::RightBrace, .. })) {
                                panic!("expected right brace");
                            }

                            Expr::Interp {
                                exprs: vec![expr],
                                strings: vec!["".to_owned(), "".to_owned()],
                            }
                        }
                        t => Expr::Literal(Value::String(t.lexeme)),
                    };

                    exprs.push(expr);
                }

                segments.push(exprs);

                if self.lexer.peek().is_none() {
                    break;
                }

                if {
                    let t = self.lexer.peek().unwrap();
                    t.is_cmd_op() || t.kind == TokenKind::Newline
                } {
                    break;
                }

                if self.lexer.peek().unwrap().kind == TokenKind::Space {
                    self.lexer.next();
                }
            }

            Cmd::Atom(segments)
        };

        lhs
    }
}

impl Token {
    fn is_cmd_op(&self) -> bool {
        use TokenKind::*;

        [
            PipePipe, AmperAmper, Semicolon,
            Pipe, StarPipe, AmperPipe,
            Great, StarGreat, AmperGreat,
            Less, StarLess, AmperLess,
        ].contains(&self.kind)
    }
}
