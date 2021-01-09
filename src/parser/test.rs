use crate::ast::{BinaryOp, Cmd, CmdOp, Expr, Value};
use crate::lexer::new as new_lexer;

use super::*;

fn make_parser(source: &str) -> Parser {
    let lexer = new_lexer(source.to_owned());
    Parser::new(lexer)
}

fn parse_expression(source: &str) -> Expr {
    make_parser(source).parse_expr(0)
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
        Stmt::Cmd(Cmd::Atom(vec![vec![Expr::Literal(Value::String("cmd1".to_owned()))]])),
        Stmt::Cmd(Cmd::Atom(vec![vec![Expr::Literal(Value::String("cmd2".to_owned()))]])),
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
        Stmt::Cmd(Cmd::Atom(vec![vec![
            Expr::Literal(Value::String("cmd".to_owned())),
            Expr::Literal(Value::String(".".to_owned())),
            Expr::Literal(Value::String("exe".to_owned())),
        ]])),
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
    assert_eq!(parse("x.foo\n=2"), vec![
        Stmt::Cmd(Cmd::Atom(vec![vec![
            Expr::Literal(Value::String("x".to_owned())),
            Expr::Literal(Value::String(".".to_owned())),
            Expr::Literal(Value::String("foo".to_owned())),
        ]])),
        Stmt::Cmd(Cmd::Atom(vec![vec![
            Expr::Literal(Value::String("=".to_owned())),
            Expr::Literal(Value::String("2".to_owned())),
        ]])),
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

#[test]
fn parses_explicit_cmd_stmt() {
    assert_eq!(parse("foo = 1"), vec![
        Stmt::Expr(Expr::Set("foo".to_owned(), Box::new(Expr::Literal(Value::Num(1.0))))),
    ]);
    assert_eq!(parse("$ foo = 1"), vec![
        Stmt::Cmd(Cmd::Atom(vec![
            vec![Expr::Literal(Value::String("foo".to_owned()))],
            vec![Expr::Literal(Value::String("=".to_owned()))],
            vec![Expr::Literal(Value::String("1".to_owned()))],
        ])),
    ]);
}

#[test]
#[should_panic]
fn panics_instead_of_cmd_fallback() {
    parse("my_fn(\n    \"foo\",\n    \"bar\",\n)");
}

#[test]
fn parses_var_decl() {
    assert_eq!(parse("let foo"), vec![
        Stmt::Let {
            init: None,
            is_exp: false,
            name: "foo".to_owned(),
        }
    ]);

    assert_eq!(parse("let foo = 1"), vec![
        Stmt::Let {
            init: Some(Expr::Literal(Value::Num(1.0))),
            is_exp: false,
            name: "foo".to_owned(),
        }
    ]);

    assert_eq!(parse("exp let foo"), vec![
        Stmt::Let {
            init: None,
            is_exp: true,
            name: "foo".to_owned(),
        }
    ]);
}

#[test]
fn parses_cmd_expr() {
    assert_eq!(
        parse_expression("$(foo)"),
        Expr::Cmd(Cmd::Atom(vec![vec![Expr::Literal(Value::String("foo".to_owned()))]])),
    );
}

#[test]
fn parses_parenthesized_cmd() {
    assert_eq!(parse(" foo && ( bar || baz ) "), vec![
        Stmt::Cmd(Cmd::Op(
            Box::new(Cmd::Atom(vec![vec![Expr::Literal(Value::String("foo".to_owned()))]])),
            CmdOp::And,
            Box::new(Cmd::Op(
                Box::new(Cmd::Atom(vec![vec![Expr::Literal(Value::String("bar".to_owned()))]])),
                CmdOp::Or,
                Box::new(Cmd::Atom(vec![vec![Expr::Literal(Value::String("baz".to_owned()))]])),
            )),
        ))
    ]);
}
