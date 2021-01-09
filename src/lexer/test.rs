use super::*;

use crate::token::{Token, TokenKind};

fn make_lexer(source: &str) -> Lexer {
    new(source.to_owned())
}

fn scan(source: &str) -> Vec<Token> {
    let lexer = make_lexer(source);
    lexer.collect()
}

#[test]
fn scans_empty_string() {
    assert_eq!(scan(""), vec![]);
}

#[test]
fn scans_spaces() {
    assert_eq!(scan("   "), vec![
        Token {
            kind: TokenKind::Space,
            lexeme: "   ".to_owned(),
        }
    ]);
}

#[test]
fn scans_keywords() {
    assert_eq!(scan("while for return continue"), vec![
        Token {
            kind: TokenKind::While,
            lexeme: "while".to_owned(),
        },
        Token {
            kind: TokenKind::Space,
            lexeme: " ".to_owned(),
        },
        Token {
            kind: TokenKind::For,
            lexeme: "for".to_owned(),
        },
        Token {
            kind: TokenKind::Space,
            lexeme: " ".to_owned(),
        },
        Token {
            kind: TokenKind::Return,
            lexeme: "return".to_owned(),
        },
        Token {
            kind: TokenKind::Space,
            lexeme: " ".to_owned(),
        },
        Token {
            kind: TokenKind::Continue,
            lexeme: "continue".to_owned(),
        }
    ]);
}

#[test]
#[should_panic]
fn panics_unexpected_symbol() {
    scan("§");
}

#[test]
fn scans_identifiers() {
    assert_eq!(scan("whilee"), vec![
        Token {
            kind: TokenKind::Identifier("whilee".to_owned()),
            lexeme: "whilee".to_owned(),
        }
    ]);
}

#[test]
fn scans_simple_string_literal() {
    assert_eq!(scan("\"hello world\""), vec![
        Token {
            kind: TokenKind::String {
                value: "hello world".to_owned(),
                does_interp: false,
            },
            lexeme: "\"hello world\"".to_owned(),
        }
    ]);
}

#[test]
fn scans_string_literal_with_escape_chars() {
    assert_eq!(scan("\"\\n\\t\\r\\\\\""), vec![
        Token {
            kind: TokenKind::String {
                value: "\n\t\r\\".to_owned(),
                does_interp: false,
            },
            lexeme: "\"\\n\\t\\r\\\\\"".to_owned(),
        }
    ]);
}

#[test]
fn scans_number_literals() {
    assert_eq!(scan("12 3.14 10. .5"), vec![
        Token {
            kind: TokenKind::Num(12.0),
            lexeme: "12".to_owned(),
        },
        Token {
            kind: TokenKind::Space,
            lexeme: " ".to_owned(),
        },
        Token {
            kind: TokenKind::Num(3.14),
            lexeme: "3.14".to_owned(),
        },
        Token {
            kind: TokenKind::Space,
            lexeme: " ".to_owned(),
        },
        Token {
            kind: TokenKind::Num(10.0),
            lexeme: "10.".to_owned(),
        },
        Token {
            kind: TokenKind::Space,
            lexeme: " ".to_owned(),
        },
        Token {
            kind: TokenKind::Num(0.5),
            lexeme: ".5".to_owned(),
        }
    ]);
}

#[test]
fn scans_interpolated_string() {
    assert_eq!(scan("\"a{for}b\""), vec![
        Token {
            kind: TokenKind::String {
                value: "a".to_owned(),
                does_interp: true,
            },
            lexeme: "\"a{".to_owned(),
        },
        Token {
            kind: TokenKind::For,
            lexeme: "for".to_owned(),
        },
        Token {
            kind: TokenKind::String {
                value: "b".to_owned(),
                does_interp: false,
            },
            lexeme: "}b\"".to_owned(),
        }
    ]);
}

#[test]
fn scans_interpolated_string_trimmed() {
    assert_eq!(scan("\"{for}\""), vec![
        Token {
            kind: TokenKind::String {
                value: "".to_owned(),
                does_interp: true,
            },
            lexeme: "\"{".to_owned(),
        },
        Token {
            kind: TokenKind::For,
            lexeme: "for".to_owned(),
        },
        Token {
            kind: TokenKind::String {
                value: "".to_owned(),
                does_interp: false,
            },
            lexeme: "}\"".to_owned(),
        }
    ]);
}

#[test]
fn scans_interpolated_string_empty() {
    assert_eq!(scan("\"a{}b\""), vec![
        Token {
            kind: TokenKind::String {
                value: "a".to_owned(),
                does_interp: true,
            },
            lexeme: "\"a{".to_owned(),
        },
        Token {
            kind: TokenKind::String {
                value: "b".to_owned(),
                does_interp: false,
            },
            lexeme: "}b\"".to_owned(),
        }
    ]);
}

#[test]
fn scans_interpolated_string_nested() {
    assert_eq!(scan("\"a{\"b{for}b\"}a\""), vec![
        Token {
            kind: TokenKind::String {
                value: "a".to_owned(),
                does_interp: true,
            },
            lexeme: "\"a{".to_owned(),
        },
        Token {
            kind: TokenKind::String {
                value: "b".to_owned(),
                does_interp: true,
            },
            lexeme: "\"b{".to_owned(),
        },
        Token {
            kind: TokenKind::For,
            lexeme: "for".to_owned(),
        },
        Token {
            kind: TokenKind::String {
                value: "b".to_owned(),
                does_interp: false,
            },
            lexeme: "}b\"".to_owned(),
        },
        Token {
            kind: TokenKind::String {
                value: "a".to_owned(),
                does_interp: false,
            },
            lexeme: "}a\"".to_owned(),
        }
    ]);
}

#[test]
fn scans_interpolated_string_dict() {
    assert_eq!(scan("\"a{{x:1}}b\""), vec![
        Token {
            kind: TokenKind::String {
                value: "a".to_owned(),
                does_interp: true,
            },
            lexeme: "\"a{".to_owned(),
        },
        Token {
            kind: TokenKind::LeftBrace,
            lexeme: "{".to_owned(),
        },
        Token {
            kind: TokenKind::Identifier("x".to_owned()),
            lexeme: "x".to_owned(),
        },
        Token {
            kind: TokenKind::Colon,
            lexeme: ":".to_owned(),
        },
        Token {
            kind: TokenKind::Num(1.0),
            lexeme: "1".to_owned(),
        },
        Token {
            kind: TokenKind::RightBrace,
            lexeme: "}".to_owned(),
        },
        Token {
            kind: TokenKind::String {
                value: "b".to_owned(),
                does_interp: false,
            },
            lexeme: "}b\"".to_owned(),
        }
    ]);
}

#[test]
fn scans_lexemes() {
    let source = "for.while:\nret\nurn  cc    whine&&!==++--break,continue;(){}[]exp+=-=*=/=\
    %=^=^%*/-+true:h;\n\nfalse!nilvar/if else;fn::==  \n =for  \"abc{222{a:2,\nc:3}}ccc{}\"\"\n\"\
    \"while\"";
    let source_materialized: String = scan(source)
        .into_iter()
        .map(|tok| tok.lexeme)
        .collect::<Vec<String>>()
        .concat();

    assert_eq!(source, source_materialized);
}

#[test]
fn peeks() {
    let mut lexer = make_lexer("+-");

    assert_eq!(lexer.peek(), Some(&Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Minus, lexeme: "-".to_owned()}));
    assert_eq!(lexer.next(), None);
}

#[test]
fn records() {
    let mut lexer = make_lexer("+-");

    lexer.start_recording();

    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Minus, lexeme: "-".to_owned()}));

    lexer.stop_recording(true);

    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Minus, lexeme: "-".to_owned()}));
    assert_eq!(lexer.next(), None);
}

#[test]
fn records_no_playback() {
    let mut lexer = make_lexer("+-^");

    lexer.start_recording();

    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Minus, lexeme: "-".to_owned()}));

    lexer.stop_recording(false);

    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Caret, lexeme: "^".to_owned()}));
    assert_eq!(lexer.next(), None);
}

#[test]
fn newline_while_peeking() {
    let mut lexer = make_lexer("+\n-");

    assert_eq!(lexer.is_new_line, true);
    assert_eq!(lexer.peek(), Some(&Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    assert_eq!(lexer.is_new_line, true);
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    assert_eq!(lexer.is_new_line, false);
    assert_eq!(lexer.peek(), Some(&Token{kind: TokenKind::Newline, lexeme: "\n".to_owned()}));
    assert_eq!(lexer.is_new_line, false);
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Newline, lexeme: "\n".to_owned()}));
    assert_eq!(lexer.is_new_line, true);
    assert_eq!(lexer.peek(), Some(&Token{kind: TokenKind::Minus, lexeme: "-".to_owned()}));
    assert_eq!(lexer.is_new_line, true);
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Minus, lexeme: "-".to_owned()}));
    assert_eq!(lexer.is_new_line, false);
    assert_eq!(lexer.next(), None);
}

#[test]
fn newline_while_recording() {
    let mut lexer = make_lexer("+-\n+");

    lexer.start_recording();

    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Minus, lexeme: "-".to_owned()}));
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Newline, lexeme: "\n".to_owned()}));

    lexer.stop_recording(true);

    assert_eq!(lexer.is_new_line, true);
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    assert_eq!(lexer.is_new_line, false);
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Minus, lexeme: "-".to_owned()}));
    assert_eq!(lexer.is_new_line, false);
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Newline, lexeme: "\n".to_owned()}));
    assert_eq!(lexer.is_new_line, true);
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    assert_eq!(lexer.is_new_line, false);
    assert_eq!(lexer.next(), None);
}

#[test]
fn records_with_peeked() {
    let mut lexer = make_lexer("+");

    assert_eq!(lexer.peek(), Some(&Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    lexer.start_recording();
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    lexer.stop_recording(true);
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    assert_eq!(lexer.next(), None);
}

#[test]
fn peeks_while_recording() {
    let mut lexer = make_lexer("+-");

    lexer.start_recording();
    assert_eq!(lexer.peek(), Some(&Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    lexer.stop_recording(true);
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Minus, lexeme: "-".to_owned()}));
    assert_eq!(lexer.next(), None);
}

#[test]
fn peeks_while_recording_and_consumes() {
    let mut lexer = make_lexer("+-");

    lexer.start_recording();
    assert_eq!(lexer.peek(), Some(&Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    lexer.stop_recording(true);

    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Minus, lexeme: "-".to_owned()}));
    assert_eq!(lexer.next(), None);
}

#[test]
fn peeks_after_record() {
    let mut lexer = make_lexer("+-");

    lexer.start_recording();
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    lexer.stop_recording(true);
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));

    assert_eq!(lexer.peek(), Some(&Token{kind: TokenKind::Minus, lexeme: "-".to_owned()}));
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Minus, lexeme: "-".to_owned()}));
    assert_eq!(lexer.next(), None);
}

#[test]
fn peeks_after_record_without_playback() {
    let mut lexer = make_lexer("+-");

    lexer.start_recording();
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    lexer.stop_recording(false);

    assert_eq!(lexer.peek(), Some(&Token{kind: TokenKind::Minus, lexeme: "-".to_owned()}));
    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Minus, lexeme: "-".to_owned()}));
    assert_eq!(lexer.next(), None);
}

#[test]
fn peeks_while_recording_nothing() {
    let mut lexer = make_lexer("+");

    lexer.start_recording();
    assert_eq!(lexer.peek(), Some(&Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    lexer.stop_recording(true);

    assert_eq!(lexer.next(), Some(Token{kind: TokenKind::Plus, lexeme: "+".to_owned()}));
    assert_eq!(lexer.next(), None);
}

#[test]
fn consumes_whitespace() {
    assert_eq!(scan(" \n \n   "), vec![
        Token{kind: TokenKind::Space, lexeme: " ".to_owned()},
        Token{kind: TokenKind::Newline, lexeme: "\n".to_owned()},
        Token{kind: TokenKind::Space, lexeme: " ".to_owned()},
        Token{kind: TokenKind::Newline, lexeme: "\n".to_owned()},
        Token{kind: TokenKind::Space, lexeme: "   ".to_owned()},
    ]);
}

#[test]
fn consumes_whitespace_recording() {
    let mut lexer = make_lexer(" \n \n   ");

    lexer.start_recording();
    lexer.by_ref().for_each(drop);
    lexer.stop_recording(true);

    assert_eq!(lexer.collect::<Vec<Token>>(), vec![
        Token{kind: TokenKind::Space, lexeme: " ".to_owned()},
        Token{kind: TokenKind::Newline, lexeme: "\n".to_owned()},
        Token{kind: TokenKind::Space, lexeme: " ".to_owned()},
        Token{kind: TokenKind::Newline, lexeme: "\n".to_owned()},
        Token{kind: TokenKind::Space, lexeme: "   ".to_owned()},
    ]);
}
