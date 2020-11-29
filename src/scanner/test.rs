use super::*;

fn scan(source: &str) -> Vec<Token> {
    let mut scanner = Scanner::new(source);
    scanner.scan()
}

#[test]
fn ignores_whitespace() {
    assert_eq!(scan("    \r\n."), vec![Token::Dot, Token::Eof]);
}

#[test]
fn scans_empty_string() {
    assert_eq!(scan(""), vec![Token::Eof]);
}

#[test]
fn scans_keyword() {
    assert_eq!(scan("while"), vec![Token::While, Token::Eof]);
}

#[test]
fn stops_scanning() {
    assert_eq!(scan("."), vec![Token::Dot, Token::Eof]);
}

#[test]
#[should_panic]
fn panics_unexpected_symbol() {
    scan("§");
}

#[test]
fn scans_identifier() {
    assert_eq!(scan("columbia"), vec![Token::Identifier {name: "columbia".to_owned()}, Token::Eof]);
}

#[test]
fn scans_complex_string() {
    use Token::*;

    let source = "for.while:return cc whine&&!==++--break,continue;(){}[]exp+=-=*=/=%=^=^%*/-+\
    true:h;false!nil?var/if else;fn::== =for";
    assert_eq!(scan(source), vec![
        For,
        Dot,
        While,
        Colon,
        Return,
        Identifier {name: "cc".to_owned()},
        Identifier {name: "whine".to_owned()},
        AmperAmper,
        BangEqual,
        Equal,
        PlusPlus,
        MinusMinus,
        Break,
        Comma,
        Continue,
        Semicolon,
        LeftParen,
        RightParen,
        LeftBrace,
        RightBrace,
        LeftBracket,
        RightBracket,
        Exp,
        PlusEqual,
        MinusEqual,
        StarEqual,
        SlashEqual,
        PercEqual,
        CaretEqual,
        Caret,
        Perc,
        Star,
        Slash,
        Minus,
        Plus,
        True,
        Colon,
        Identifier {name: "h".to_owned()},
        Semicolon,
        False,
        Bang,
        Nil,
        Question,
        Var,
        Slash,
        If,
        Else,
        Semicolon,
        Fn,
        Colon,
        Colon,
        EqualEqual,
        Equal,
        For,
        Eof,
    ]);
}

#[test]
fn scans_simple_string_literal() {
    assert_eq!(scan("\"hello world\""), vec![Token::String {
        value: "hello world".to_owned(),
        does_interp: false,
        begins_cmd: false,
        ends_cmd: false,
    }, Token::Eof]);
}

#[test]
fn scans_string_literal_with_escape_chars() {
    assert_eq!(scan("\"\\n\\t\\r\\\\\""), vec![Token::String {
        value: "\n\t\r\\".to_owned(),
        does_interp: false,
        begins_cmd: false,
        ends_cmd: false,
    }, Token::Eof]);
}

#[test]
fn scans_number_literals() {
    assert_eq!(scan("12 3634 3333 3.14 10. .5"), vec![
        Token::Num {
            value: 12.0,
        },
        Token::Num {
            value: 3634.0,
        },
        Token::Num {
            value: 3333.0,
        },
        Token::Num {
            value: 3.14,
        },
        Token::Num {
            value: 10.0,
        },
        Token::Num {
            value: 0.5,
        },
        Token::Eof,
    ]);
}

#[test]
fn scans_simple_interpolated_string() {
    assert_eq!(scan("\"a{1}b\""), vec![
        Token::String {
            value: "a".to_owned(),
            does_interp: true,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::Num {
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
    assert_eq!(scan("\"{1}a{2+2}b{3}\""), vec![
        Token::String {
            value: "".to_owned(),
            does_interp: true,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::Num {
            value: 1.0,
        },
        Token::String {
            value: "a".to_owned(),
            does_interp: true,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::Num {
            value: 2.0,
        },
        Token::Plus,
        Token::Num {
            value: 2.0,
        },
        Token::String {
            value: "b".to_owned(),
            does_interp: true,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::Num {
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
    assert_eq!(scan("\"{1}\""), vec![
        Token::String {
            value: "".to_owned(),
            does_interp: true,
            begins_cmd: false,
            ends_cmd: false,
        },
        Token::Num {
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
    assert_eq!(scan("\"a{}b{}\""), vec![
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
    assert_eq!(scan("\"l1{\"l2{\"l3\"+\"innermost\"}\"}l1end\""), vec![
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
    assert_eq!(scan("$(docker ps)"), vec![
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
    assert_eq!(scan("$(aaa)"), vec![
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
    assert_eq!(scan("$({111})"), vec![
        Token::String {
            value: "".to_owned(),
            does_interp: true,
            begins_cmd: true,
            ends_cmd: false,
        },
        Token::Num {
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
    assert_eq!(scan("$(a \"b $() b\" c)"), vec![
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
    assert_eq!(scan("$(a{1}a b{2} b c {3}c d {4} d)"), vec![
        Token::String {
            value: "a".to_owned(),
            does_interp: true,
            begins_cmd: true,
            ends_cmd: false,
        },
        Token::Num {
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
        Token::Num {
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
        Token::Num {
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
        Token::Num {
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
    assert_eq!(scan("$(\"ccc{111}ccc\")"), vec![
        Token::String {
            value: "ccc".to_owned(),
            does_interp: true,
            begins_cmd: true,
            ends_cmd: false,
        },
        Token::Num {
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
    assert_eq!(scan("$({$(a)})"), vec![
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

#[test]
fn scans_var_decl() {
    assert_eq!(scan("var abc;"), vec![
        Token::Var,
        Token::Identifier {
            name: "abc".to_owned(),
        },
        Token::Semicolon,
        Token::Eof,
    ]);
}

#[test]
fn scans_var_decl_initialized() {
    assert_eq!(scan("var abc = 123;"), vec![
        Token::Var,
        Token::Identifier {
            name: "abc".to_owned(),
        },
        Token::Equal,
        Token::Num {
            value: 123.0,
        },
        Token::Semicolon,
        Token::Eof,
    ]);
}

#[test]
fn scans_block() {
    assert_eq!(scan("{1;2;3;}"), vec![
        Token::LeftBrace,
        Token::Num{value: 1.0},
        Token::Semicolon,
        Token::Num{value: 2.0},
        Token::Semicolon,
        Token::Num{value: 3.0},
        Token::Semicolon,
        Token::RightBrace,
        Token::Eof,
    ]);
}

#[test]
fn scans_if() {
    assert_eq!(scan("if true { 1; }"), vec![
        Token::If,
        Token::True,
        Token::LeftBrace,
        Token::Num{value: 1.0},
        Token::Semicolon,
        Token::RightBrace,
        Token::Eof,
    ]);
}

#[test]
fn scans_if_else() {
    assert_eq!(scan("if true { 1; } else { 2; }"), vec![
        Token::If,
        Token::True,
        Token::LeftBrace,
        Token::Num{value: 1.0},
        Token::Semicolon,
        Token::RightBrace,
        Token::Else,
        Token::LeftBrace,
        Token::Num{value: 2.0},
        Token::Semicolon,
        Token::RightBrace,
        Token::Eof,
    ]);
}

#[test]
fn scans_if_elseif() {
    assert_eq!(scan("if true { 1; } else if true { 2; }"), vec![
        Token::If,
        Token::True,
        Token::LeftBrace,
        Token::Num{value: 1.0},
        Token::Semicolon,
        Token::RightBrace,
        Token::Else,
        Token::If,
        Token::True,
        Token::LeftBrace,
        Token::Num{value: 2.0},
        Token::Semicolon,
        Token::RightBrace,
        Token::Eof,
    ]);
}

#[test]
fn scans_if_elseif_else() {
    assert_eq!(scan("if true { 1; } else if true { 2; } else { 3; }"), vec![
        Token::If,
        Token::True,
        Token::LeftBrace,
        Token::Num{value: 1.0},
        Token::Semicolon,
        Token::RightBrace,
        Token::Else,
        Token::If,
        Token::True,
        Token::LeftBrace,
        Token::Num{value: 2.0},
        Token::Semicolon,
        Token::RightBrace,
        Token::Else,
        Token::LeftBrace,
        Token::Num{value: 3.0},
        Token::Semicolon,
        Token::RightBrace,
        Token::Eof,
    ]);
}
