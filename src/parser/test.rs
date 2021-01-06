use crate::ast::{BinaryOp, Expr, UnaryOp, Value};
use crate::lexer::new as new_lexer;

use super::*;

fn make_parser(source: &str) -> Parser {
    let lexer = new_lexer(source.to_owned());
    Parser::new(lexer)
}

fn parse_expression(source: &str) -> Expr {
    make_parser(source).parse_expression(0)
}

fn parse(source: &str) -> Vec<Stmt> {
    make_parser(source).parse()
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

#[test]
fn parses_cmd_stmt() {
    assert_eq!(parse("cmd1\ncmd2"), vec![
        Stmt::Cmd(vec![vec![Expr::Literal(Value::String("cmd1".to_owned()))]]),
        Stmt::Cmd(vec![vec![Expr::Literal(Value::String("cmd2".to_owned()))]]),
    ]);
}

#[test]
fn parses_assignment_stmt() {
    assert_eq!(parse("foo = \n    1"), vec![
        Stmt::Expr(Expr::Set("foo".to_owned(), Box::new(Expr::Literal(Value::Num(1.0)))))
    ]);
}

#[test]
fn parses_call_stmt() {
    assert_eq!(parse("my_fn(\n    \"foo\",\n    \"bar\"\n)"), vec![
        Stmt::Expr(Expr::Call {
            func: Box::new(Expr::Get("my_fn".to_owned())),
            args: vec![
                Expr::Literal(Value::String("foo".to_owned())),
                Expr::Literal(Value::String("bar".to_owned())),
            ],
        }),
    ]);
}

#[test]
fn parses_cmd_stmt_with_dot() {
    assert_eq!(parse("cmd.exe"), vec![
        Stmt::Cmd(vec![vec![Expr::Literal(Value::String("cmd.exe".to_owned()))]]),
    ]);
}

#[test]
fn parses_assignment_stmt_with_dots() {
    assert_eq!(parse("x.foo = 1"), vec![
        Stmt::Expr(Expr::SetField {
            base: Box::new(Expr::Get("x".to_owned())),
            index: Box::new(Expr::Literal(Value::String("foo".to_owned()))),
            value: Box::new(Expr::Literal(Value::Num(1.0))),
        }),
    ]);
}

#[test]
fn parses_call_stmt_with_dots() {
    assert_eq!(parse("x.foo()"), vec![
        Stmt::Expr(Expr::Call {
            func: Box::new(Expr::GetField {
                base: Box::new(Expr::Get("x".to_owned())),
                index: Box::new(Expr::Literal(Value::String("foo".to_owned()))),
            }),
            args: vec![],
        })
    ]);
}

#[test]
fn parses_incorrect_expr_stmt_with_dots() {
    assert_eq!(parse("x.foo\n()"), vec![
        Stmt::Cmd(vec![vec![Expr::Literal(Value::String("x.foo".to_owned()))]]),
        Stmt::Cmd(vec![vec![Expr::Literal(Value::String("()".to_owned()))]]),
    ]);
}

#[test]
fn parses_expr_stmt_continuation() {
    assert_eq!(parse("foo() foo\n=1"), vec![
        Stmt::Expr(Expr::Call {
            func: Box::new(Expr::Get("foo".to_owned())),
            args: vec![],
        }),
        Stmt::Expr(Expr::Set("foo".to_owned(), Box::new(Expr::Literal(Value::Num(1.0))))),
    ]);
}
