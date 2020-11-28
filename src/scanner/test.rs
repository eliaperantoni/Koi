use super::*;

#[test]
fn ignores_whitespace() {
    let scanner = Scanner::new("    \r\n.");
    assert_eq!(scanner.scan(), vec![Token::Dot, Token::Eof]);
}

#[test]
fn scans_empty_string() {
    let scanner = Scanner::new("");
    assert_eq!(scanner.scan(), vec![Token::Eof]);
}

#[test]
fn scans_keyword() {
    let scanner = Scanner::new("while");
    assert_eq!(scanner.scan(), vec![Token::While, Token::Eof]);
}

#[test]
fn stops_scanning() {
    let scanner = Scanner::new(".");
    assert_eq!(scanner.scan(), vec![Token::Dot, Token::Eof]);
}

#[test]
#[should_panic]
fn panics_unexpected_symbol() {
    let scanner = Scanner::new("§");
    scanner.scan();
}

#[test]
fn scans_identifier() {
    let scanner = Scanner::new("columbia");
    assert_eq!(scanner.scan(), vec![Token::Identifier, Token::Eof]);
}

#[test]
fn scans_complex_string() {
    use Token::*;

    let scanner = Scanner::new("for.while:return cc whine&&!==++--break,continue\
    ;(){}[]exp+=-=*=/=%=^=^%*/-+true:h;false!nil?var/if else;fn::== =for");
    assert_eq!(scanner.scan(), vec![
        For, Dot, While, Colon, Return, Identifier, Identifier, AmperAmper, BangEqual, Equal, PlusPlus,
        MinusMinus, Break, Comma, Continue, Semicolon, LeftParen, RightParen, LeftBrace, RightBrace,
        LeftBracket, RightBracket, Exp, PlusEqual, MinusEqual, StarEqual, SlashEqual, PercEqual,
        CaretEqual, Caret, Perc, Star, Slash, Minus, Plus, True, Colon, Identifier, Semicolon, False,
        Bang, Nil, Question, Var, Slash, If, Else, Semicolon, Fn, Colon, Colon, EqualEqual, Equal, For,
        Eof,
    ]);
}

#[test]
fn scans_simple_string_literal() {
    let scanner = Scanner::new("\"hello world\"");
    assert_eq!(scanner.scan(), vec![Token::String {
        value: "hello world".to_owned(),
        does_interp: false,
        begins_cmd: false,
        ends_cmd: false,
    }, Token::Eof]);
}

#[test]
fn scans_string_literal_with_escape_chars() {
    let scanner = Scanner::new("\"\\n\\t\\r\\\\\"");
    assert_eq!(scanner.scan(), vec![Token::String {
        value: "\n\t\r\\".to_owned(),
        does_interp: false,
        begins_cmd: false,
        ends_cmd: false,
    }, Token::Eof]);
}

#[test]
fn scans_number_literals() {
    let scanner = Scanner::new("12 3634 3333 3.14 10. .5");
    assert_eq!(scanner.scan(), vec![
        Token::Number {
            value: 12.0,
        },
        Token::Number {
            value: 3634.0,
        },
        Token::Number {
            value: 3333.0,
        },
        Token::Number {
            value: 3.14,
        },
        Token::Number {
            value: 10.0,
        },
        Token::Number {
            value: 0.5,
        },
        Token::Eof,
    ]);
}

#[test]
fn scans_simple_interpolated_string() {
    let scanner = Scanner::new("\"a{1}b\"");
    assert_eq!(scanner.scan(), vec![
        Token::String {
            value: "a".to_owned(),
            does_interp: true,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::Number {
            value: 1.0,
        },
        Token::String {
            value: "b".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::Eof,
    ]);
}

#[test]
fn scans_complex_interpolated_string() {
    let scanner = Scanner::new("\"{1}a{2+2}b{3}\"");
    assert_eq!(scanner.scan(), vec![
        Token::String {
            value: "".to_owned(),
            does_interp: true,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::Number {
            value: 1.0,
        },
        Token::String {
            value: "a".to_owned(),
            does_interp: true,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::Number {
            value: 2.0,
        },
        Token::Plus,
        Token::Number {
            value: 2.0,
        },
        Token::String {
            value: "b".to_owned(),
            does_interp: true,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::Number {
            value: 3.0,
        },
        Token::String {
            value: "".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::Eof,
    ]);
}

#[test]
fn scans_completely_interpolated_string() {
    let scanner = Scanner::new("\"{1}\"");
    assert_eq!(scanner.scan(), vec![
        Token::String {
            value: "".to_owned(),
            does_interp: true,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::Number {
            value: 1.0,
        },
        Token::String {
            value: "".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::Eof,
    ]);
}


// This is wrong and the issue will be picked up by the parser but it's nice that the scanner keeps
// working as intended even in these edge cases
#[test]
fn scans_empty_interpolation() {
    let scanner = Scanner::new("\"a{}b{}\"");
    assert_eq!(scanner.scan(), vec![
        Token::String {
            value: "a".to_owned(),
            does_interp: true,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::String {
            value: "b".to_owned(),
            does_interp: true,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::String {
            value: "".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::Eof,
    ]);
}

#[test]
fn scans_nested_interpolated_string() {
    let scanner = Scanner::new("\"l1{\"l2{\"l3\"+\"innermost\"}\"}l1end\"");
    assert_eq!(scanner.scan(), vec![
        Token::String {
            value: "l1".to_owned(),
            does_interp: true,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::String {
            value: "l2".to_owned(),
            does_interp: true,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::String {
            value: "l3".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::Plus,
        Token::String {
            value: "innermost".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::String {
            value: "".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::String {
            value: "l1end".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::Eof,
    ]);
}

#[test]
fn scans_simple_command() {
    let scanner = Scanner::new("$(docker ps)");
    assert_eq!(scanner.scan(), vec![
        Token::String {
            value: "docker".to_owned(),
            does_interp: false,
            begins_cmd: true,
            ends_cmd: false,
        },
        Token::String {
            value: "ps".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: true,
        },
        Token::Eof,
    ]);
}

#[test]
fn scans_mono_string_command() {
    let scanner = Scanner::new("$(aaa)");
    assert_eq!(scanner.scan(), vec![
        Token::String {
            value: "aaa".to_owned(),
            does_interp: false,
            begins_cmd: true,
            ends_cmd: true,
        },
        Token::Eof,
    ]);
}

#[test]
fn scans_mono_string_interpolated_command() {
    let scanner = Scanner::new("$({111})");
    assert_eq!(scanner.scan(), vec![
        Token::String {
            value: "".to_owned(),
            does_interp: true,
            begins_cmd: true,
            ends_cmd: false,
        },
        Token::Number {
            value: 111.0,
        },
        Token::String {
            value: "".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: true,
        },
        Token::Eof,
    ]);
}

#[test]
fn scans_command_with_string_literal() {
    let scanner = Scanner::new("$(a \"b $() b\" c)");
    assert_eq!(scanner.scan(), vec![
        Token::String {
            value: "a".to_owned(),
            does_interp: false,
            begins_cmd: true,
            ends_cmd: false,
        },
        Token::String {
            value: "b $() b".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::String {
            value: "c".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: true,
        },
        Token::Eof,
    ]);
}

#[test]
fn scans_command_with_interpolation() {
    let scanner = Scanner::new("$(a{1}a b{2} b c {3}c d {4} d)");
    assert_eq!(scanner.scan(), vec![
        Token::String {
            value: "a".to_owned(),
            does_interp: true,
            begins_cmd: true,
            ends_cmd: false,
        },
        Token::Number {
            value: 1.0,
        },
        Token::String {
            value: "a".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::String {
            value: "b".to_owned(),
            does_interp: true,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::Number {
            value: 2.0,
        },
        Token::String {
            value: "".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::String {
            value: "b".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::String {
            value: "c".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::String {
            value: "".to_owned(),
            does_interp: true,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::Number {
            value: 3.0,
        },
        Token::String {
            value: "c".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::String {
            value: "d".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::String {
            value: "".to_owned(),
            does_interp: true,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::Number {
            value: 4.0,
        },
        Token::String {
            value: "".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::String {
            value: "d".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: true,
        },
        Token::Eof,
    ]);
}

#[test]
fn scans_command_with_interpolated_string_literal() {
    let scanner = Scanner::new("$(\"ccc{111}ccc\")");
    assert_eq!(scanner.scan(), vec![
        Token::String {
            value: "ccc".to_owned(),
            does_interp: true,
            begins_cmd: true,
            ends_cmd: false,
        },
        Token::Number {
            value: 111.0,
        },
        Token::String {
            value: "ccc".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: true,
        },
        Token::Eof,
    ]);
}

#[test]
fn scans_command_with_interpolated_command() {
    let scanner = Scanner::new("$({$(a)})");
    assert_eq!(scanner.scan(), vec![
        Token::String {
            value: "".to_owned(),
            does_interp: true,
            begins_cmd: true,
            ends_cmd: false,
        },
        Token::String {
            value: "a".to_owned(),
            does_interp: false,
            begins_cmd: true,
            ends_cmd: true,
        },
        Token::String {
            value: "".to_owned(),
            does_interp: false,
            begins_cmd: false,
            ends_cmd: true,
        },
        Token::Eof,
    ]);
}
