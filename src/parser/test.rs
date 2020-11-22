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
    assert_eq!(parse("1 + 2"), Expr::Bin {
        lhs: Box::from(Expr::Value(Value::Int(1))),
        op: Token::Plus,
        rhs: Box::from(Expr::Value(Value::Int(2))),
    });
}

#[test]
fn parses_correct_precedence() {
    assert_eq!(parse("1 + 2 * 3"), Expr::Bin {
        lhs: Box::from(Expr::Value(Value::Int(1))),
        op: Token::Plus,
        rhs: Box::from(Expr::Bin {
            lhs: Box::from(Expr::Value(Value::Int(2))),
            op: Token::Star,
            rhs: Box::from(Expr::Value(Value::Int(3))),
        }),
    });
}

#[test]
fn parses_correct_associativity() {
    assert_eq!(parse("1 + 2 + 3"), Expr::Bin {
        lhs: Box::from(Expr::Bin {
            lhs: Box::from(Expr::Value(Value::Int(1))),
            op: Token::Plus,
            rhs: Box::from(Expr::Value(Value::Int(2))),
        }),
        op: Token::Plus,
        rhs: Box::from(Expr::Value(Value::Int(3))),
    });
}
