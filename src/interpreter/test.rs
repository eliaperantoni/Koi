use super::*;
use crate::scanner::Scanner;
use crate::parser::Parser;

fn eval(source: &str) -> Value {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan();

    let mut parser = Parser::new(tokens);
    let prog = parser.parse();

    let expr = if let Stmt::Expr(expr) = &prog[0] {
        expr
    } else {
        panic!("expected a single expression")
    };

    let mut interpreter = Interpreter::new();
    interpreter.eval(expr)
}

#[test]
fn interprets_arithmetic_expressions() {
    assert_eq!(eval("2"), Value::Num(2.0));
    assert_eq!(eval("2 + 2"), Value::Num(4.0));
    assert_eq!(eval("10 * 5 - 20"), Value::Num(30.0));
    assert_eq!(eval("((100 % 99) ^ 2 * 10 - 5 + (12 * 2)) / 2"), Value::Num(14.5));
}

#[test]
fn interprets_boolean_expression() {
    assert_eq!(eval("true || false && true && (false || true)"), Value::Bool(true));
}

#[test]
fn interprets_interpolated_string() {
    assert_eq!(eval("\"a{\"b\"}c{12.2}d{true}e\""), Value::String("abc12.2dtruee".to_owned()));
}
