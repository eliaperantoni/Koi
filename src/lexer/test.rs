use super::*;

fn scan(source: &str) -> Vec<Token> {
    let lexer = Lexer::new(source.to_owned());
    lexer.collect()
}

#[test]
fn scans_empty_string() {
    assert_eq!(scan(""), vec![
        Token {
            kind: TokenKind::Eof,
            lexeme: "".to_owned(),
        }
    ]);
}

#[test]
fn scans_keyword() {
    assert_eq!(scan("while"), vec![
        Token {
            kind: TokenKind::While,
            lexeme: "while".to_owned(),
        },
        Token {
            kind: TokenKind::Eof,
            lexeme: "".to_owned(),
        }
    ]);
}

#[test]
#[should_panic]
fn panics_unexpected_symbol() {
    scan("§");
}

#[test]
fn scans_identifier() {
    assert_eq!(scan("whilee"), vec![
        Token {
            kind: TokenKind::Identifier {
                name: "whilee".to_owned(),
            },
            lexeme: "whilee".to_owned(),
        },
        Token {
            kind: TokenKind::Eof,
            lexeme: "".to_owned(),
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
        },
        Token {
            kind: TokenKind::Eof,
            lexeme: "".to_owned(),
        }
    ]);
}
