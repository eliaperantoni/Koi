use super::*;

use crate::scanner::Scanner;

fn parse_expr(source: &str) -> Expr {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan();

    let mut parser = Parser::new(tokens);
    parser.parse_expr(0)
}

#[test]
fn parses_literal() {
    assert_eq!(parse_expr("12"), Expr::Value(Value::Num(12.0)));
}

#[test]
fn parses_sum() {
    assert_eq!(parse_expr("1 + 2"), Expr::Binary {
        lhs: Expr::Value(Value::Num(1.0)).into(),
        op: Token::Plus,
        rhs: Expr::Value(Value::Num(2.0)).into(),
    });
}

#[test]
fn parses_power() {
    assert_eq!(parse_expr("1 ^ 2 ^ 3"), Expr::Binary {
        lhs: Expr::Value(Value::Num(1.0)).into(),
        op: Token::Caret,
        rhs: Expr::Binary {
            lhs: Expr::Value(Value::Num(2.0)).into(),
            op: Token::Caret,
            rhs: Expr::Value(Value::Num(3.0)).into(),
        }.into(),
    });
}

#[test]
fn parses_logical() {
    assert_eq!(parse_expr("1 || 2 && 3"), Expr::Binary {
        lhs: Expr::Value(Value::Num(1.0)).into(),
        op: Token::PipePipe,
        rhs: Expr::Binary {
            lhs: Expr::Value(Value::Num(2.0)).into(),
            op: Token::AmperAmper,
            rhs: Expr::Value(Value::Num(3.0)).into(),
        }.into(),
    });
}

#[test]
fn parses_correct_precedence() {
    assert_eq!(parse_expr("1 + 2 * 3"), Expr::Binary {
        lhs: Expr::Value(Value::Num(1.0)).into(),
        op: Token::Plus,
        rhs: Expr::Binary {
            lhs: Expr::Value(Value::Num(2.0)).into(),
            op: Token::Star,
            rhs: Expr::Value(Value::Num(3.0)).into(),
        }.into(),
    });
}

#[test]
fn parses_correct_associativity() {
    assert_eq!(parse_expr("1 + 2 + 3"), Expr::Binary {
        lhs: Expr::Binary {
            lhs: Expr::Value(Value::Num(1.0)).into(),
            op: Token::Plus,
            rhs: Expr::Value(Value::Num(2.0)).into(),
        }.into(),
        op: Token::Plus,
        rhs: Expr::Value(Value::Num(3.0)).into(),
    });
}


#[test]
fn parses_unary() {
    assert_eq!(parse_expr("+1"), Expr::Unary {
        rhs: Expr::Value(Value::Num(1.0)).into(),
        op: Token::Plus,
    });
}

#[test]
fn parses_nested_unary() {
    assert_eq!(parse_expr("+-1"), Expr::Unary {
        rhs: Expr::Unary {
            rhs: Expr::Value(Value::Num(1.0)).into(),
            op: Token::Minus,
        }.into(),
        op: Token::Plus,
    });
}

#[test]
fn parses_complex_unary() {
    assert_eq!(parse_expr("+++---!1"), Expr::Unary {
        rhs: Expr::Unary {
            rhs: Expr::Unary {
                rhs: Expr::Unary {
                    rhs: Expr::Unary {
                        rhs: Expr::Value(Value::Num(1.0)).into(),
                        op: Token::Bang,
                    }.into(),
                    op: Token::Minus,
                }.into(),
                op: Token::MinusMinus,
            }.into(),
            op: Token::Plus,
        }.into(),
        op: Token::PlusPlus,
    });
}

#[test]
fn parses_nested_among_binary() {
    assert_eq!(parse_expr("-1 + -2"), Expr::Binary {
        lhs: Expr::Unary {
            rhs: Expr::Value(Value::Num(1.0)).into(),
            op: Token::Minus,
        }.into(),
        op: Token::Plus,
        rhs: Expr::Unary {
            rhs: Expr::Value(Value::Num(2.0)).into(),
            op: Token::Minus,
        }.into(),
    });
}

#[test]
fn parses_complex_expr() {
    assert_eq!(parse_expr("1 = 5 *= 2 + 4 % 3 ^ -5 || !1"), Expr::Binary {
        lhs: Expr::Value(Value::Num(1.0)).into(),
        op: Token::Equal,
        rhs: Expr::Binary {
            lhs: Expr::Value(Value::Num(5.0)).into(),
            op: Token::StarEqual,
            rhs: Expr::Binary {
                lhs: Expr::Binary {
                    lhs: Expr::Value(Value::Num(2.0)).into(),
                    op: Token::Plus,
                    rhs: Expr::Binary {
                        lhs: Expr::Value(Value::Num(4.0)).into(),
                        op: Token::Perc,
                        rhs: Expr::Binary {
                            lhs: Expr::Value(Value::Num(3.0)).into(),
                            op: Token::Caret,
                            rhs: Expr::Unary {
                                op: Token::Minus,
                                rhs: Expr::Value(Value::Num(5.0)).into(),
                            }.into(),
                        }.into(),
                    }.into(),
                }.into(),
                op: Token::PipePipe,
                rhs: Expr::Unary {
                    op: Token::Bang,
                    rhs: Expr::Value(Value::Num(1.0)).into(),
                }.into(),
            }.into(),
        }.into(),
    });
}

#[test]
fn parses_parenthesized() {
    assert_eq!(parse_expr("1 + (5 * 2)"), Expr::Binary {
        lhs: Expr::Value(Value::Num(1.0)).into(),
        op: Token::Plus,
        rhs: Expr::Binary {
            lhs: Expr::Value(Value::Num(5.0)).into(),
            op: Token::Star,
            rhs: Expr::Value(Value::Num(2.0)).into(),
        }.into(),
    });
}

#[test]
fn parses_bool_expr() {
    assert_eq!(parse_expr("true || false && true && (false || true)"), Expr::Binary {
        lhs: Expr::Value(Value::Bool(true)).into(),
        op: Token::PipePipe,
        rhs: Expr::Binary {
            lhs: Expr::Binary {
                lhs: Expr::Value(Value::Bool(false)).into(),
                op: Token::AmperAmper,
                rhs: Expr::Value(Value::Bool(true)).into(),
            }.into(),
            op: Token::AmperAmper,
            rhs: Expr::Binary {
                lhs: Expr::Value(Value::Bool(false)).into(),
                op: Token::PipePipe,
                rhs: Expr::Value(Value::Bool(true)).into(),
            }.into(),
        }.into(),
    });
}

#[test]
fn parses_string_literal() {
    assert_eq!(parse_expr("\"abc\""), Expr::Value(Value::String("abc".to_owned())).into());
}

#[test]
fn parses_interpolated_string() {
    assert_eq!(parse_expr("\"abc{1}def{2}ghi\""), Expr::Interp {
        segments: vec!["abc".to_owned(), "def".to_owned(), "ghi".to_owned()],
        exprs: vec![Expr::Value(Value::Num(1.0)).into(), Expr::Value(Value::Num(2.0)).into()],
    }.into());
}

#[test]
fn parses_interpolated_prefix_string() {
    assert_eq!(parse_expr("\"{1}abc\""), Expr::Interp {
        segments: vec!["".to_owned(), "abc".to_owned()],
        exprs: vec![Expr::Value(Value::Num(1.0)).into()],
    }.into());
}

#[test]
fn parses_interpolated_suffix_string() {
    assert_eq!(parse_expr("\"abc{1}\""), Expr::Interp {
        segments: vec!["abc".to_owned(), "".to_owned()],
        exprs: vec![Expr::Value(Value::Num(1.0)).into()],
    }.into());
}
