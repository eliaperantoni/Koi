use crate::ast::{BinaryOp, Cmd, CmdOp, Expr};
use crate::lexer::new as new_lexer;
use crate::interp::{Func, Value};

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

#[test]
fn parses_cmd_expr_stmt() {
    assert_eq!(parse("$(\n    foo\n    bar\n)"), vec![
        Stmt::Expr(Expr::Cmd(Cmd::Atom(vec![
            vec![Expr::Literal(Value::String("foo".to_owned()))],
            vec![Expr::Literal(Value::String("bar".to_owned()))],
        ]))),
    ]);
}

#[test]
fn parses_cmd_with_interpolation() {
    assert_eq!(parse("a{1}b a{1} {1}b"), vec![
        Stmt::Cmd(Cmd::Atom(vec![
            vec![
                Expr::Literal(Value::String("a".to_owned())),
                Expr::Literal(Value::Num(1.0)),
                Expr::Literal(Value::String("b".to_owned())),
            ],
            vec![
                Expr::Literal(Value::String("a".to_owned())),
                Expr::Literal(Value::Num(1.0)),
            ],
            vec![
                Expr::Literal(Value::Num(1.0)),
                Expr::Literal(Value::String("b".to_owned())),
            ],
        ])),
    ]);
}

#[test]
fn parses_cmd_with_escaping() {
    // a"b"c{"d"}e
    assert_eq!(parse("a\"b\"c\"{\"d\"}\"e"), vec![
        Stmt::Cmd(Cmd::Atom(vec![vec![
            Expr::Literal(Value::String("a".to_owned())),
            Expr::Literal(Value::String("b".to_owned())),
            Expr::Literal(Value::String("c".to_owned())),
            Expr::Interp {
                strings: vec!["".to_owned(), "".to_owned()],
                exprs: vec![Expr::Literal(Value::String("d".to_owned()))],
            },
            Expr::Literal(Value::String("e".to_owned()))
        ]]))
    ]);
}

#[test]
fn parses_return() {
    assert_eq!(parse("return"), vec![
        Stmt::Return(None),
    ]);

    assert_eq!(parse("return 1"), vec![
        Stmt::Return(Some(Expr::Literal(Value::Num(1.0)))),
    ]);

    assert_eq!(parse("return\n1"), vec![
        Stmt::Return(None),
        Stmt::Cmd(Cmd::Atom(vec![vec![
            Expr::Literal(Value::String("1".to_owned())),
        ]]))
    ]);
}

#[test]
fn parses_continue() {
    assert_eq!(parse("continue"), vec![
        Stmt::Continue,
    ]);
}

#[test]
fn parses_break() {
    assert_eq!(parse("break"), vec![
        Stmt::Break,
    ]);
}

#[test]
fn parses_if() {
    assert_eq!(parse("if true {\ncmd_if_true\n}"), vec![
        Stmt::If {
            cond: Expr::Literal(Value::Bool(true)),
            then_do: Box::new(Stmt::Block(vec![
                Stmt::Cmd(Cmd::Atom(vec![vec![
                    Expr::Literal(Value::String("cmd_if_true".to_owned())),
                ]])),
            ])),
            else_do: None,
        }
    ]);
}

#[test]
fn parses_if_with_else() {
    assert_eq!(parse("if true {\ncmd_if_true\n} else {\ncmd_if_false\n}"), vec![
        Stmt::If {
            cond: Expr::Literal(Value::Bool(true)),
            then_do: Box::new(Stmt::Block(vec![
                Stmt::Cmd(Cmd::Atom(vec![vec![
                    Expr::Literal(Value::String("cmd_if_true".to_owned())),
                ]])),
            ])),
            else_do: Some(Box::new(Stmt::Block(vec![
                Stmt::Cmd(Cmd::Atom(vec![vec![
                    Expr::Literal(Value::String("cmd_if_false".to_owned())),
                ]])),
            ]))),
        }
    ]);
}

#[test]
fn parses_if_with_else_and_else_if() {
    assert_eq!(parse("if true {\ncmd_a\n} else if false {\ncmd_b\n} else {\ncmd_c\n}"), vec![
        Stmt::If {
            cond: Expr::Literal(Value::Bool(true)),
            then_do: Box::new(Stmt::Block(vec![
                Stmt::Cmd(Cmd::Atom(vec![vec![
                    Expr::Literal(Value::String("cmd_a".to_owned())),
                ]])),
            ])),
            else_do: Some(Box::new(Stmt::If {
                cond: Expr::Literal(Value::Bool(false)),
                then_do: Box::new(Stmt::Block(vec![
                    Stmt::Cmd(Cmd::Atom(vec![vec![
                        Expr::Literal(Value::String("cmd_b".to_owned())),
                    ]])),
                ])),
                else_do: Some(Box::new(Stmt::Block(vec![
                    Stmt::Cmd(Cmd::Atom(vec![vec![
                        Expr::Literal(Value::String("cmd_c".to_owned())),
                    ]])),
                ]))),
            })),
        }
    ]);
}

#[test]
fn parses_for() {
    assert_eq!(parse("for \n i \n \n in \n foo \n {}"), vec![
        Stmt::For {
            lvar: "i".to_owned(),
            rvar: None,
            iterated: Expr::Get("foo".to_owned()),
            each_do: Box::new(Stmt::Block(vec![])),
        }
    ]);
}

#[test]
fn parses_foreach() {
    assert_eq!(parse("for \n x \n , \n y \n in \n foo \n {}"), vec![
        Stmt::For {
            lvar: "x".to_owned(),
            rvar: Some("y".to_owned()),
            iterated: Expr::Get("foo".to_owned()),
            each_do: Box::new(Stmt::Block(vec![])),
        }
    ]);
}

#[test]
fn parses_while() {
    assert_eq!(parse("while \n true \n { \n }"), vec![
        Stmt::While {
            cond: Expr::Literal(Value::Bool(true)),
            then_do: Box::new(Stmt::Block(vec![])),
        }
    ]);
}

#[test]
fn parses_fn() {
    assert_eq!(parse("fn foo \n( x , y , z ) \n {}"), vec![
        Stmt::Func(Func::User {
            name: Some("foo".to_owned()),
            params: vec!["x".to_owned(), "y".to_owned(), "z".to_owned()],
            body: Box::new(Stmt::Block(vec![])),
        })
    ]);
}

#[test]
fn parses_fn_no_params() {
    assert_eq!(parse("fn foo \n() \n {}"), vec![
        Stmt::Func(Func::User {
            name: Some("foo".to_owned()),
            params: vec![],
            body: Box::new(Stmt::Block(vec![])),
        })
    ]);
}

#[test]
fn parses_cmd_semicolon() {
    assert_eq!(parse("cmd1 ; cmd2"), vec![
        Stmt::Cmd(Cmd::Op(
            Box::new(Cmd::Atom(vec![vec![
                Expr::Literal(Value::String("cmd1".to_owned())),
            ]])),
            CmdOp::Seq,
            Box::new(Cmd::Atom(vec![vec![
                Expr::Literal(Value::String("cmd2".to_owned())),
            ]])),
        ))
    ]);
}

#[test]
fn parses_lambda() {
    assert_eq!(parse("print(fn(){})"), vec![
        Stmt::Expr(Expr::Call {
            func: Box::new(Expr::Get("print".to_owned())),
            args: vec![
                Expr::Lambda(Func::User {
                    name: None,
                    params: vec![],
                    body: Box::new(Stmt::Block(vec![])),
                })
            ],
        })
    ]);
}

#[test]
fn parses_block() {
    assert_eq!(parse("{nop()}"), vec![
        Stmt::Block(vec![
            Stmt::Expr(
                Expr::Call {
                    func: Box::new(Expr::Get("nop".to_owned())),
                    args: vec![],
                }
            )
        ])
    ]);
}
