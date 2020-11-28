use super::*;
use crate::scanner::Scanner;
use crate::parser::Parser;

fn eval(source: &str) -> Value {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan();

    let mut parser = Parser::new(tokens);
    let expr = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.eval(&expr)
}

#[test]
fn interprets_scalar() {
    assert_eq!(eval("2"), Value::Number(2.0));
}

#[test]
fn interprets_sum() {
    assert_eq!(eval("2 + 2"), Value::Number(4.0));
}

#[test]
fn interprets_non_trivial_arithmetic() {
    assert_eq!(eval("10 * 5 - 20"), Value::Number(30.0));
}
