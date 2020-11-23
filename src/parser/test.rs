use super::*;

use crate::scanner::Scanner;

fn parse(source: &str) -> Expr {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan();

    let mut parser = Parser::new(tokens);
    parser.parse()
}

#[test]
fn parses_literal() {
    assert_eq!(parse("12"), Expr::Value(Value::Int(12)));
}

#[test]
fn parses_sum() {
    assert_eq!(parse("1 + 2"), Expr::Binary {
        lhs: Box::from(Expr::Value(Value::Int(1))),
        op: Token::Plus,
        rhs: Box::from(Expr::Value(Value::Int(2))),
    });
}

#[test]
fn parses_correct_precedence() {
    assert_eq!(parse("1 + 2 * 3"), Expr::Binary {
        lhs: Box::from(Expr::Value(Value::Int(1))),
        op: Token::Plus,
        rhs: Box::from(Expr::Binary {
            lhs: Box::from(Expr::Value(Value::Int(2))),
            op: Token::Star,
            rhs: Box::from(Expr::Value(Value::Int(3))),
        }),
    });
}

#[test]
fn parses_correct_associativity() {
    assert_eq!(parse("1 + 2 + 3"), Expr::Binary {
        lhs: Box::from(Expr::Binary {
            lhs: Box::from(Expr::Value(Value::Int(1))),
            op: Token::Plus,
            rhs: Box::from(Expr::Value(Value::Int(2))),
        }),
        op: Token::Plus,
        rhs: Box::from(Expr::Value(Value::Int(3))),
    });
}


#[test]
fn parses_unary() {
    assert_eq!(parse("+1"), Expr::Unary {
        rhs: Box::from(Expr::Value(Value::Int(1))),
        op: Token::Plus,
    });
}

#[test]
fn parses_nested_unary() {
    assert_eq!(parse("+-1"), Expr::Unary {
        rhs: Box::from(Expr::Unary {
            rhs: Box::from(Expr::Value(Value::Int(1))),
            op: Token::Minus,
        }),
        op: Token::Plus,
    });
}

#[test]
fn parses_nested_among_binary() {
    assert_eq!(parse("-1 + -2"), Expr::Binary {
        lhs: Expr::Unary {
            rhs: Box::from(Expr::Value(Value::Int(1))),
            op: Token::Minus,
        }.into(),
        op: Token::Plus,
        rhs: Expr::Unary {
            rhs: Box::from(Expr::Value(Value::Int(2))),
            op: Token::Minus,
        }.into(),
    });
}
