use crate::ast::{Expr, Value};
use crate::scanner::Token;
use std::hint::unreachable_unchecked;

#[cfg(test)]
mod test;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn eval(&self, expr: &Expr) -> Value {
        match expr {
            Expr::Value(value) => value.clone(),

            Expr::Unary { op, rhs } => {
                let rhs = self.eval(rhs);

                match op {
                    Token::Plus => rhs,
                    Token::Minus => {
                        if let Value::Num(number) = rhs {
                            Value::Num(-number)
                        } else {
                            panic!("bad operand, expected number");
                        }
                    }
                    _ => panic!("bad op {:?}", op),
                }
            }

            Expr::Binary { lhs, op, rhs } => {
                let lhs = self.eval(lhs);
                let rhs = self.eval(rhs);

                match op {
                    Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::Perc | Token::Caret => {
                        if let (Value::Num(lhs), Value::Num(rhs)) = (lhs, rhs) {
                            Value::Num(match op {
                                Token::Plus => lhs + rhs,
                                Token::Minus => lhs - rhs,
                                Token::Star => lhs * rhs,
                                Token::Slash => lhs / rhs,
                                Token::Perc => lhs % rhs,
                                Token::Caret => lhs.powf(rhs),
                                _ => unreachable!(),
                            })
                        } else {
                            panic!("bad operands, expected numbers");
                        }
                    }
                    _ => panic!("bad op {:?}", op),
                }
            }
        }
    }
}
