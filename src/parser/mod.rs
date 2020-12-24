use crate::lexer::Lexer;
use crate::ast::*;
use crate::token::{Token, TokenKind};

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer,
        }
    }

    pub fn parse_expression(&mut self, min_bp: u8) -> Expr {
        let mut lhs = match self.lexer.next() {
            Some(Token { kind: TokenKind::Num(num), .. }) => Expr::Literal(Value::Num(num)),
            Some(Token { kind: TokenKind::Identifier(name), .. }) => Expr::Get(name),

            Some(Token { kind: TokenKind::String { value, does_interp: false }, .. }) => Expr::Literal(Value::String(value)),
            Some(Token { kind: TokenKind::String { value, does_interp: true }, .. }) => {
                let mut strings = Vec::new();
                let mut exprs = Vec::new();

                strings.push(value);

                loop {
                    exprs.push(self.parse_expression(0));

                    if let Some(Token { kind: TokenKind::String { value, does_interp }, .. }) = self.lexer.next() {
                        strings.push(value);

                        if !does_interp {
                            break;
                        }
                    } else {
                        panic!("bad token");
                    }
                }

                Expr::Interp { strings, exprs }
            }

            Some(Token { kind: TokenKind::True, .. }) => Expr::Literal(Value::Bool(true)),
            Some(Token { kind: TokenKind::False, .. }) => Expr::Literal(Value::Bool(false)),

            Some(Token { kind: TokenKind::Nil, .. }) => Expr::Literal(Value::Nil),

            _ => panic!("bad token"),
        };

        lhs
    }
}

fn infix_binding_power(op: char) -> (u8, u8) {
    match op {
        '+' | '-' => (1, 2),
        '*' | '/' => (3, 4),
        _ => panic!("bad op: {:?}"),
    }
}
