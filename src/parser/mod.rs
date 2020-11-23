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
        let lhs = self.advance();
        let mut lhs = match lhs {
            Token::Int { value } => Expr::Value(Value::Int(value)),
            Token::Plus | Token::Minus => {
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
            let op = match self.peek() {
                Token::Eof => break,
                Token::Plus | Token::Minus | Token::Star | Token::Slash => self.peek(),
                t @ _ => panic!("bad token {:?}", t),
            };

            let (l_bp, r_bp) = infix_binding_power(&op);
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
        }

        lhs
    }
}

fn prefix_binding_power(op: &Token) -> ((), u8) {
    match op {
        Token::Plus | Token::Minus => ((), 5),
        _ => panic!("bad op {:?}", op),
    }
}

fn infix_binding_power(op: &Token) -> (u8, u8) {
    match op {
        Token::Plus | Token::Minus => (1, 2),
        Token::Star | Token::Slash => (3, 4),
        _ => panic!("bad op {:?}", op),
    }
}
