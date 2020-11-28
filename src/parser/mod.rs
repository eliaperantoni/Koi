use crate::scanner::Token;
use crate::ast::{Value, Expr};

#[cfg(test)]
mod test;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
        }
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }

    fn advance(&mut self) -> Token {
        let token = self.peek();
        self.current += 1;
        token
    }

    pub fn parse(&mut self) -> Expr {
        self.parse_expr(0)
    }

    fn parse_expr(&mut self, min_bp: u8) -> Expr {
        use Token::*;

        let lhs = self.advance();
        let mut lhs = match lhs {
            Num { value } => Expr::Literal(Value::Num(value)),
            String { mut value, mut does_interp, .. } => {
                if !does_interp {
                    return Expr::Literal(Value::String(value.clone()));
                }

                let mut segments = Vec::new();
                let mut exprs = Vec::new();

                while does_interp {
                    segments.push(value.clone());

                    exprs.push(self.parse_expr(0));

                    if let String {
                        value: next_value,
                        does_interp: next_does_interp,
                        ..
                    } = self.advance() {
                        value = next_value;
                        does_interp = next_does_interp;
                    } else {
                        panic!("bad token, expected interpolation closing string");
                    }
                }

                segments.push(value.clone());

                Expr::Interp { segments, exprs }
            }
            True => Expr::Literal(Value::Bool(true)),
            False => Expr::Literal(Value::Bool(false)),
            LeftParen => {
                let lhs = self.parse_expr(0);
                assert_eq!(self.advance(), RightParen);
                lhs
            }
            Plus | Minus | PlusPlus | MinusMinus | Bang => {
                let ((), r_bp) = prefix_binding_power(&lhs);
                let rhs = self.parse_expr(r_bp);

                Expr::Unary {
                    rhs: Box::from(rhs),
                    op: lhs,
                }
            }
            t @ _ => panic!("bad token {:?}", t),
        };

        loop {
            let op = self.peek();

            if !(op.is_infix_op() || op == RightParen) {
                break;
            }

            if let Some((l_bp, r_bp)) = infix_binding_power(&op) {
                if l_bp < min_bp {
                    break;
                }

                self.advance();
                let rhs = self.parse_expr(r_bp);

                lhs = Expr::Binary {
                    lhs: Box::from(lhs),
                    rhs: Box::from(rhs),
                    op,
                };

                continue;
            }

            break;
        }

        lhs
    }
}

fn prefix_binding_power(op: &Token) -> ((), u8) {
    use Token::*;
    match op {
        Plus | Minus | PlusPlus | MinusMinus | Bang => ((), 15),
        _ => panic!("bad op {:?}", op),
    }
}

fn infix_binding_power(op: &Token) -> Option<(u8, u8)> {
    use Token::*;
    let res = match op {
        Caret => (18, 17),
        Star | Slash | Perc => (13, 14),
        Plus | Minus => (11, 12),
        Less | LessEqual | Greater | GreaterEqual => (9, 10),
        EqualEqual | BangEqual => (7, 8),
        AmperAmper => (5, 6),
        PipePipe => (3, 4),
        Equal | PlusEqual | MinusEqual | StarEqual | SlashEqual | PercEqual | CaretEqual => (2, 1),
        _ => return None,
    };
    Some(res)
}
