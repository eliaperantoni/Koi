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
        use Token::*;

        match expr {
            Expr::Value(value) => value.clone(),

            Expr::Unary { op, rhs } => {
                let rhs = self.eval(rhs);

                match op {
                    Plus => rhs,
                    Minus => {
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
                    Plus | Minus | Star | Slash | Perc | Caret => {
                        if let (Value::Num(lhs), Value::Num(rhs)) = (lhs, rhs) {
                            Value::Num(match op {
                                Plus => lhs + rhs,
                                Minus => lhs - rhs,
                                Star => lhs * rhs,
                                Slash => lhs / rhs,
                                Perc => lhs % rhs,
                                Caret => lhs.powf(rhs),
                                _ => unreachable!(),
                            })
                        } else {
                            panic!("bad operands, expected numbers");
                        }
                    },

                    // TODO Make boolean expressions short circuit
                    PipePipe | AmperAmper => {
                        if let (Value::Bool(lhs), Value::Bool(rhs)) = (lhs, rhs) {
                            Value::Bool(match op {
                                PipePipe => lhs || rhs,
                                AmperAmper => lhs && rhs,
                                _ => unreachable!(),
                            })
                        } else {
                            panic!("bad operands, expected bools");
                        }
                    },
                    
                    _ => panic!("bad op {:?}", op),
                }
            }
        }
    }
}
