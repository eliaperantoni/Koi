use crate::lexer::Lexer;
use crate::ast::*;
use crate::token::{Token, TokenKind};
use crate::ast::Expr::Binary;
use itertools::__std_iter::Peekable;

pub struct Parser {
    lexer: Peekable<Lexer>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer: lexer.peekable(),
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

            Some(Token { kind, .. }) if prefix_binding_power(&kind).is_some() => {
                let ((), r_bp) = prefix_binding_power(&kind).unwrap();
                let rhs = self.parse_expression(r_bp);

                make_prefix_expr(&kind, rhs)
            }

            Some(Token { kind: TokenKind::LeftParen, .. }) => {
                let lhs = self.parse_expression(0);
                assert!(matches!(self.lexer.next(), Some(Token { kind: TokenKind::RightParen, .. })));
                lhs
            }

            _ => panic!("bad token"),
        };

        loop {
            let op = match self.lexer.peek() {
                Some(Token { kind, .. }) => kind,
                None => break,
            };

            if let Some((l_bp, ())) = postfix_binding_power(op) {
                if l_bp < min_bp {
                    break;
                }

                let op = self.lexer.next().unwrap().kind;

                match op {
                    TokenKind::LeftBracket => {
                        let rhs = self.parse_expression(0);
                        lhs = Expr::GetField {
                            base: Box::new(lhs),
                            index: Box::new(rhs),
                        };
                        assert!(matches!(self.lexer.next(), Some(Token { kind: TokenKind::RightBracket, .. })));
                    }
                    TokenKind::Dot => {
                        let name = match self.lexer.next() {
                            Some(Token { kind: TokenKind::Identifier(name), .. }) => name,
                            _ => panic!("expected identifier"),
                        };
                        lhs = Expr::GetField {
                            base: Box::new(lhs),
                            index: Box::new(Expr::Literal(Value::String(name))),
                        };
                    }
                    _ => unreachable!(),
                }
                continue;
            }

            if let Some((l_bp, r_bp)) = infix_binding_power(&op) {
                if l_bp < min_bp {
                    break;
                }

                let op = self.lexer.next().unwrap().kind;

                let rhs = self.parse_expression(r_bp);
                lhs = make_infix_expr(lhs, &op, rhs);
                continue;
            }

            break;
        }

        lhs
    }
}

fn make_prefix_expr(op: &TokenKind, rhs: Expr) -> Expr {
    match *op {
        TokenKind::Plus => rhs,
        TokenKind::Minus => Expr::Unary(UnaryOp::Neg, Box::new(rhs)),
        TokenKind::Bang => Expr::Unary(UnaryOp::Not, Box::new(rhs)),
        _ => unreachable!()
    }
}

fn make_infix_expr(lhs: Expr, op: &TokenKind, rhs: Expr) -> Expr {
    let lhs = Box::new(lhs);
    let rhs = Box::new(rhs);

    let op = op;

    match *op {
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

            if matches!(op, TokenKind::BangEqual) {
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

            if matches!(op, TokenKind::GreatEqual | TokenKind::LessEqual) {
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
}

fn postfix_binding_power(op: &TokenKind) -> Option<(u8, ())> {
    use TokenKind::*;
    let bp = match op {
        LeftBracket | Dot => (5, ()),
        _ => return None,
    };
    Some(bp)
}

fn prefix_binding_power(op: &TokenKind) -> Option<((), u8)> {
    use TokenKind::*;
    let bp = match op {
        Plus | Minus => ((), 5),
        _ => return None,
    };
    Some(bp)
}

fn infix_binding_power(op: &TokenKind) -> Option<(u8, u8)> {
    use TokenKind::*;
    let bp = match op {
        Caret => (16, 15),
        Star | Slash | Perc => (13, 14),
        Plus | Minus => (11, 12),
        Great | GreatEqual | Less | LessEqual => (9, 10),
        EqualEqual | BangEqual => (7, 8),
        AmperAmper => (5, 6),
        PipePipe => (3, 4),
        Equal | PlusEqual | MinusEqual | StarEqual | SlashEqual | PercEqual | CaretEqual => (2, 1),
        _ => return None,
    };
    Some(bp)
}
