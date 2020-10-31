use super::*;

#[test]
fn ignores_whitespace() {
    let scanner = Scanner::new("    \r\n.");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![Token::Dot]);
}

#[test]
fn scans_empty_string() {
    let scanner = Scanner::new("");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![]);
}

#[test]
fn scans_keyword() {
    let scanner = Scanner::new("while");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![Token::While]);
}

#[test]
fn stops_scanning() {
    let scanner = Scanner::new(".");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![Token::Dot]);
}

#[test]
#[should_panic]
fn panics_unexpected_symbol() {
    let mut scanner = Scanner::new("§");
    scanner.next();
}

#[test]
fn scans_identifier() {
    let scanner = Scanner::new("columbia");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![Token::Identifier]);
}

#[test]
fn scans_complex_string() {
    use Token::*;

    let scanner = Scanner::new("for.while:return cc whine&&!==++--break,continue\
    ;(){}[]$()exp+=-=*=/=%=^=^%*/-+true:h;false!nil?var/if else;fn::== =for");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![
        For, Dot, While, Colon, Return, Identifier, Identifier, AmperAmper, BangEqual, Equal, PlusPlus,
        MinusMinus, Break, Comma, Continue, Semicolon, LeftParen, RightParen, LeftBrace, RightBrace,
        LeftBracket, RightBracket, DollarLeftParen, RightParen, Exp, PlusEqual, MinusEqual, StarEqual,
        SlashEqual, PercEqual, CaretEqual, Caret, Perc, Star, Slash, Minus, Plus, True, Colon, Identifier,
        Semicolon, False, Bang, Nil, Question, Var, Slash, If, Else, Semicolon, Fn, Colon, Colon,
        EqualEqual, Equal, For
    ]);
}

#[test]
fn scans_simple_string_literal() {
    let scanner = Scanner::new("\"hello world\"");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![Token::String {
        value: "hello world".to_owned(),
        does_interp: false,
    }]);
}

#[test]
fn scans_string_literal_with_escape_chars() {
    let scanner = Scanner::new("\"\\n\\t\\r\\\\\"");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![Token::String {
        value: "\n\t\r\\".to_owned(),
        does_interp: false,
    }]);
}

#[test]
fn scans_int_literals() {
    let scanner = Scanner::new("12 3634 3333");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![
        Token::Int {
            value: 12,
        },
        Token::Int {
            value: 3634,
        },
        Token::Int {
            value: 3333,
        },
    ]);
}

#[test]
fn scans_float_literal() {
    let scanner = Scanner::new("3.14 10. .5");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![
        Token::Float {
            value: 3.14,
        },
        Token::Float {
            value: 10.0,
        },
        Token::Float {
            value: 0.5,
        },
    ]);
}

#[test]
fn scans_simple_interpolated_string() {
    let scanner = Scanner::new("\"a{1}b\"");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![
        Token::String {
            value: "a".to_owned(),
            does_interp: true,
        },
        Token::Int {
            value: 1,
        },
        Token::String {
            value: "b".to_owned(),
            does_interp: false,
        },
    ]);
}

#[test]
fn scans_complex_interpolated_string() {
    let scanner = Scanner::new("\"{1}a{2+2}b{3}\"");

    let interp_count = scanner.interp_count;
    let tokens = scanner.collect::<Vec<_>>();

    assert_eq!(tokens, vec![
        Token::String {
            value: "".to_owned(),
            does_interp: true,
        },
        Token::Int {
            value: 1,
        },
        Token::String {
            value: "a".to_owned(),
            does_interp: true,
        },
        Token::Int {
            value: 2,
        },
        Token::Plus,
        Token::Int {
            value: 2,
        },
        Token::String {
            value: "b".to_owned(),
            does_interp: true,
        },
        Token::Int {
            value: 3,
        },
        Token::String {
            value: "".to_owned(),
            does_interp: false,
        },
    ]);

    assert_eq!(interp_count, 0);
}

#[test]
fn scans_completely_interpolated_string() {
    let scanner = Scanner::new("\"{1}\"");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![
        Token::String {
            value: "".to_owned(),
            does_interp: true,
        },
        Token::Int {
            value: 1,
        },
        Token::String {
            value: "".to_owned(),
            does_interp: false,
        },
    ]);
}


// This is wrong and the issue will be picked up by the parser but it's nice that the scanner keeps
// working as intended even in these edge cases
#[test]
fn scans_empty_interpolation() {
    let scanner = Scanner::new("\"a{}b{}\"");

    let interp_count = scanner.interp_count;
    let tokens = scanner.collect::<Vec<_>>();

    assert_eq!(tokens, vec![
        Token::String {
            value: "a".to_owned(),
            does_interp: true,
        },
        Token::String {
            value: "b".to_owned(),
            does_interp: true,
        },
        Token::String {
            value: "".to_owned(),
            does_interp: false,
        },
    ]);

    assert_eq!(interp_count, 0);
}

#[test]
fn scans_nested_interpolated_string() {
    let scanner = Scanner::new("\"l1{\"l2{\"l3\"+\"innermost\"}\"}l1end\"");

    let interp_count = scanner.interp_count;
    let tokens = scanner.collect::<Vec<_>>();

    assert_eq!(tokens, vec![
        Token::String {
            value: "l1".to_owned(),
            does_interp: true,
        },
        Token::String {
            value: "l2".to_owned(),
            does_interp: true,
        },
        Token::String {
            value: "l3".to_owned(),
            does_interp: false,
        },
        Token::Plus,
        Token::String {
            value: "innermost".to_owned(),
            does_interp: false,
        },
        Token::String {
            value: "".to_owned(),
            does_interp: false,
        },
        Token::String {
            value: "l1end".to_owned(),
            does_interp: false,
        },
    ]);

    assert_eq!(interp_count, 0);
}
