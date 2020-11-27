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
        lhs: Expr::Value(Value::Int(1)).into(),
        op: Token::Plus,
        rhs: Expr::Value(Value::Int(2)).into(),
    });
}

#[test]
fn parses_power() {
    assert_eq!(parse("1 ^ 2 ^ 3"), Expr::Binary {
        lhs: Expr::Value(Value::Int(1)).into(),
        op: Token::Caret,
        rhs: Expr::Binary {
            lhs: Expr::Value(Value::Int(2)).into(),
            op: Token::Caret,
            rhs: Expr::Value(Value::Int(3)).into(),
        }.into(),
    });
}

#[test]
fn parses_logical() {
    assert_eq!(parse("1 || 2 && 3"), Expr::Binary {
        lhs: Expr::Value(Value::Int(1)).into(),
        op: Token::PipePipe,
        rhs: Expr::Binary {
            lhs: Expr::Value(Value::Int(2)).into(),
            op: Token::AmperAmper,
            rhs: Expr::Value(Value::Int(3)).into(),
        }.into(),
    });
}

#[test]
fn parses_correct_precedence() {
    assert_eq!(parse("1 + 2 * 3"), Expr::Binary {
        lhs: Expr::Value(Value::Int(1)).into(),
        op: Token::Plus,
        rhs: Expr::Binary {
            lhs: Expr::Value(Value::Int(2)).into(),
            op: Token::Star,
            rhs: Expr::Value(Value::Int(3)).into(),
        }.into(),
    });
}

#[test]
fn parses_correct_associativity() {
    assert_eq!(parse("1 + 2 + 3"), Expr::Binary {
        lhs: Expr::Binary {
            lhs: Expr::Value(Value::Int(1)).into(),
            op: Token::Plus,
            rhs: Expr::Value(Value::Int(2)).into(),
        }.into(),
        op: Token::Plus,
        rhs: Expr::Value(Value::Int(3)).into(),
    });
}


#[test]
fn parses_unary() {
    assert_eq!(parse("+1"), Expr::Unary {
        rhs: Expr::Value(Value::Int(1)).into(),
        op: Token::Plus,
    });
}

#[test]
fn parses_nested_unary() {
    assert_eq!(parse("+-1"), Expr::Unary {
        rhs: Expr::Unary {
            rhs: Expr::Value(Value::Int(1)).into(),
            op: Token::Minus,
        }.into(),
        op: Token::Plus,
    });
}

#[test]
fn parses_complex_unary() {
    assert_eq!(parse("+++---!1"), Expr::Unary {
        rhs: Expr::Unary {
            rhs: Expr::Unary {
                rhs: Expr::Unary {
                    rhs: Expr::Unary {
                        rhs: Expr::Value(Value::Int(1)).into(),
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
    assert_eq!(parse("-1 + -2"), Expr::Binary {
        lhs: Expr::Unary {
            rhs: Expr::Value(Value::Int(1)).into(),
            op: Token::Minus,
        }.into(),
        op: Token::Plus,
        rhs: Expr::Unary {
            rhs: Expr::Value(Value::Int(2)).into(),
            op: Token::Minus,
        }.into(),
    });
}

#[test]
fn parses_complex_expr() {
    assert_eq!(parse("1 = 5 *= 2 + 4 % 3 ^ -5 || !1"), Expr::Binary {
        lhs: Expr::Value(Value::Int(1)).into(),
        op: Token::Equal,
        rhs: Expr::Binary {
            lhs: Expr::Value(Value::Int(5)).into(),
            op: Token::StarEqual,
            rhs: Expr::Binary {
                lhs: Expr::Binary {
                    lhs: Expr::Value(Value::Int(2)).into(),
                    op: Token::Plus,
                    rhs: Expr::Binary {
                        lhs: Expr::Value(Value::Int(4)).into(),
                        op: Token::Perc,
                        rhs: Expr::Binary {
                            lhs: Expr::Value(Value::Int(3)).into(),
                            op: Token::Caret,
                            rhs: Expr::Unary {
                                op: Token::Minus,
                                rhs: Expr::Value(Value::Int(5)).into(),
                            }.into(),
                        }.into(),
                    }.into(),
                }.into(),
                op: Token::PipePipe,
                rhs: Expr::Unary {
                    op: Token::Bang,
                    rhs: Expr::Value(Value::Int(1)).into(),
                }.into(),
            }.into(),
        }.into(),
    });
}
