use crate::ast::{BinaryOp, Expr, UnaryOp, Value};

use super::*;

fn make_parser(source: &str) -> Parser {
    let lexer = Lexer::new(source.to_owned());
    Parser::new(lexer)
}

fn parse_expression(source: &str) -> Expr {
    make_parser(source).parse_expression(0)
}

#[test]
fn parses_literals() {
    for (source, want) in &[
        ("1.2", Expr::Literal(Value::Num(1.2))),
        ("\"foo\"", Expr::Literal(Value::String("foo".to_owned()))),
        ("true", Expr::Literal(Value::Bool(true))),
        ("false", Expr::Literal(Value::Bool(false))),
        ("nil", Expr::Literal(Value::Nil)),
    ] {
        assert_eq!(parse_expression(source), *want);
    }
}

#[test]
fn parses_lookups() {
    for (source, want) in &[
        ("foo", Expr::Get("foo".to_owned())),
        ("foo.bar", Expr::GetField {
            base: Box::new(Expr::Get("foo".to_owned())),
            index: Box::new(Expr::Literal(Value::String("bar".to_owned()))),
        }),
        ("foo[\"bar\"]", Expr::GetField {
            base: Box::new(Expr::Get("foo".to_owned())),
            index: Box::new(Expr::Literal(Value::String("bar".to_owned()))),
        }),
        ("foo[1]", Expr::GetField {
            base: Box::new(Expr::Get("foo".to_owned())),
            index: Box::new(Expr::Literal(Value::Num(1.0))),
        }),
        ("foo.bar[\"baz\"].egg[\"beam\"]", Expr::GetField {
            base: Box::new(Expr::GetField {
                base: Box::new(Expr::GetField {
                    base: Box::new(Expr::GetField {
                        base: Box::new(Expr::Get("foo".to_owned())),
                        index: Box::new(Expr::Literal(Value::String("bar".to_owned()))),
                    }),
                    index: Box::new(Expr::Literal(Value::String("baz".to_owned()))),
                }),
                index: Box::new(Expr::Literal(Value::String("egg".to_owned()))),
            }),
            index: Box::new(Expr::Literal(Value::String("beam".to_owned()))),
        }),
    ] {
        assert_eq!(parse_expression(source), *want);
    }
}

#[test]
fn parses_assignments() {
    for (source, want) in &[
        ("foo=5", Expr::Set("foo".to_owned(), Box::new(Expr::Literal(Value::Num(5.0))))),
        ("foo.bar=5", Expr::SetField {
            base: Box::new(Expr::Get("foo".to_owned())),
            index: Box::new(Expr::Literal(Value::String("bar".to_owned()))),
            value: Box::new(Expr::Literal(Value::Num(5.0))),
        }),
        ("foo[\"bar\"]=5", Expr::SetField {
            base: Box::new(Expr::Get("foo".to_owned())),
            index: Box::new(Expr::Literal(Value::String("bar".to_owned()))),
            value: Box::new(Expr::Literal(Value::Num(5.0))),
        }),
        ("foo[1]=5", Expr::SetField {
            base: Box::new(Expr::Get("foo".to_owned())),
            index: Box::new(Expr::Literal(Value::Num(1.0))),
            value: Box::new(Expr::Literal(Value::Num(5.0))),
        }),
        ("foo.bar[\"baz\"].egg[\"beam\"]=5", Expr::SetField {
            base: Box::new(Expr::GetField {
                base: Box::new(Expr::GetField {
                    base: Box::new(Expr::GetField {
                        base: Box::new(Expr::Get("foo".to_owned())),
                        index: Box::new(Expr::Literal(Value::String("bar".to_owned()))),
                    }),
                    index: Box::new(Expr::Literal(Value::String("baz".to_owned()))),
                }),
                index: Box::new(Expr::Literal(Value::String("egg".to_owned()))),
            }),
            index: Box::new(Expr::Literal(Value::String("beam".to_owned()))),
            value: Box::new(Expr::Literal(Value::Num(5.0))),
        }),
    ] {
        assert_eq!(parse_expression(source), *want);
    }
}

#[test]
fn parses_precedence() {
    assert_eq!(parse_expression("1+2*3"), Expr::Binary(
        Box::new(Expr::Literal(Value::Num(1.0))),
        BinaryOp::Sum,
        Box::new(Expr::Binary(
            Box::new(Expr::Literal(Value::Num(2.0))),
            BinaryOp::Mul,
            Box::new(Expr::Literal(Value::Num(3.0))),
        )),
    ));
}

#[test]
fn parses_associativity() {
    assert_eq!(parse_expression("1+2+3"), Expr::Binary(
        Box::new(Expr::Binary(
            Box::new(Expr::Literal(Value::Num(1.0))),
            BinaryOp::Sum,
            Box::new(Expr::Literal(Value::Num(2.0))),
        )),
        BinaryOp::Sum,
        Box::new(Expr::Literal(Value::Num(3.0))),
    ));
}
