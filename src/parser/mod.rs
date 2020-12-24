use crate::lexer::Lexer;
use crate::ast::*;
use crate::token::{Token, TokenKind};
use crate::ast::Expr::Binary;

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

        loop {
            let op = self.lexer.next();
            let op = match op {
                Some(Token { kind, .. }) if kind.is_infix() => kind,
                None => break,
                _ => panic!("bad token"),
            };

            let (l_bp, r_bp) = infix_binding_power(&op);
            if l_bp < min_bp {
                break;
            }

            let rhs = self.parse_expression(r_bp);

            lhs = {
                let lhs = Box::new(lhs);
                let rhs = Box::new(rhs);

                match op {
                    TokenKind::Plus => Expr::Binary(lhs, BinaryOp::Sum, rhs),
                    TokenKind::Minus => Expr::Binary(lhs, BinaryOp::Sub, rhs),
                    TokenKind::Star => Expr::Binary(lhs, BinaryOp::Mul, rhs),
                    TokenKind::Slash => Expr::Binary(lhs, BinaryOp::Div, rhs),
                    TokenKind::Perc => Expr::Binary(lhs, BinaryOp::Mod, rhs),
                    TokenKind::Caret => Expr::Binary(lhs, BinaryOp::Pow, rhs),

                    TokenKind::AmperAmper => Expr::Binary(lhs, BinaryOp::And, rhs),
                    TokenKind::PipePipe => Expr::Binary(lhs, BinaryOp::Or, rhs),

                    TokenKind::EqualEqual | TokenKind::BangEqual => {
                        let mut expr = Expr::Binary(lhs, BinaryOp::Equal, rhs);

                        if TokenKind::BangEqual == op {
                            expr = Expr::Unary(UnaryOp::Not, Box::new(expr));
                        }

                        expr
                    }

                    TokenKind::Great | TokenKind::GreatEqual | TokenKind::Less | TokenKind::LessEqual => {
                        let mut expr = Expr::Binary(
                            lhs.clone(),
                            match op {
                                TokenKind::Great | TokenKind::GreatEqual => BinaryOp::Great,
                                TokenKind::Less | TokenKind::LessEqual => BinaryOp::Less,
                                _ => unreachable!(),
                            },
                            rhs.clone(),
                        );

                        if [TokenKind::GreatEqual, TokenKind::LessEqual].contains(&op) {
                            let lhs = lhs.clone();
                            let rhs = rhs.clone();

                            expr = Expr::Binary(
                                Box::new(expr),
                                BinaryOp::Or,
                                Box::new(Expr::Binary(lhs, BinaryOp::Equal, rhs)),
                            );
                        }

                        expr
                    }

                    _ => unreachable!(),
                }
            };
        }

        lhs
    }
}

fn infix_binding_power(op: &TokenKind) -> (u8, u8) {
    use TokenKind::*;
    match op {
        Caret => (16, 15),
        Star | Slash | Perc => (13, 14),
        Plus | Minus => (11, 12),
        Great | GreatEqual | Less | LessEqual => (9, 10),
        EqualEqual | BangEqual => (7, 8),
        AmperAmper => (5, 6),
        PipePipe => (3, 4),
        Equal | PlusEqual | MinusEqual | StarEqual | SlashEqual | PercEqual | CaretEqual => (2, 1),
        _ => panic!("bad op"),
    }
}
