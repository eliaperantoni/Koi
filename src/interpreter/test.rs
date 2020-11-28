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
fn interprets_arithmetic_expression() {
    assert_eq!(eval("2"), Value::Number(2.0));
    assert_eq!(eval("2 + 2"), Value::Number(4.0));
    assert_eq!(eval("10 * 5 - 20"), Value::Number(30.0));
    assert_eq!(eval("((100 % 99) ^ 2 * 10 - 5 + (12 * 2)) / 2"), Value::Number(14.5));
}
