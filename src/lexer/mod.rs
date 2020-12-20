use crate::token::{Token, TokenKind};
use itertools::Itertools;

pub struct Lexer {
    source: Vec<char>,
    cursor: usize,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source: source.chars().collect(),
            cursor: 0,
        }
    }

    fn peek_at(&self, offset: usize) -> Option<char> {
        if self.cursor + offset >= self.source.len() {
            return None;
        }

        Some(self.source[self.cursor + offset])
    }

    fn make_lexeme(&self, from: usize, to: usize) -> String {
        (&self.source[from..to]).iter().collect()
    }

    fn scan_symbol(&mut self) -> Token {
        let (kind, length) = match self.peek_at(0).unwrap() {
            ',' => (TokenKind::Comma, 1),
            '.' => (TokenKind::Dot, 1),
            ':' => (TokenKind::Colon, 1),
            ';' => (TokenKind::Semicolon, 1),

            ' ' => {
                let mut length = 1;
                loop {
                    match self.peek_at(length) {
                        Some(' ') => length += 1,
                        _ => break,
                    }
                }
                (TokenKind::Space, length)
            }
            '\n' => (TokenKind::Newline, 1),

            '(' => (TokenKind::LeftParen, 1),
            ')' => (TokenKind::RightParen, 1),
            '[' => (TokenKind::LeftBracket, 1),
            ']' => (TokenKind::RightBracket, 1),
            '{' => (TokenKind::LeftBrace, 1),
            '}' => (TokenKind::RightBrace, 1),

            // Chars that may only appear by themselves or followed by an equals sign
            '!' | '=' | '/' | '^' | '%' | '>' | '<' => {
                let (kind, equal_kind) = match self.peek_at(0).unwrap() {
                    '!' => (TokenKind::Bang, TokenKind::BangEqual),
                    '=' => (TokenKind::Equal, TokenKind::EqualEqual),
                    '/' => (TokenKind::Slash, TokenKind::SlashEqual),
                    '^' => (TokenKind::Caret, TokenKind::CaretEqual),
                    '%' => (TokenKind::Perc, TokenKind::PercEqual),
                    '>' => (TokenKind::Great, TokenKind::GreatEqual),
                    '<' => (TokenKind::Less, TokenKind::LessEqual),
                    _ => unreachable!(),
                };

                if let Some('=') = self.peek_at(1) {
                    (equal_kind, 2)
                } else {
                    (kind, 1)
                }
            }

            '+' => match self.peek_at(1) {
                Some('+') => (TokenKind::PlusPlus, 2),
                Some('=') => (TokenKind::PlusEqual, 2),
                _ => (TokenKind::Plus, 1),
            },
            '-' => match self.peek_at(1) {
                Some('-') => (TokenKind::MinusMinus, 2),
                Some('=') => (TokenKind::MinusEqual, 2),
                _ => (TokenKind::Minus, 1),
            },

            '*' => match self.peek_at(1) {
                Some('=') => (TokenKind::StarEqual, 2),
                Some('>') => (TokenKind::StarGreat, 2),
                Some('<') => (TokenKind::StarLess, 2),
                Some('|') => (TokenKind::StarPipe, 2),
                _ => (TokenKind::Star, 1),
            },

            '|' => match self.peek_at(1) {
                Some('|') => (TokenKind::PipePipe, 2),
                _ => (TokenKind::Pipe, 1),
            },

            '&' => match self.peek_at(1) {
                Some('&') => (TokenKind::AmperAmper, 2),
                Some('>') => (TokenKind::AmperGreat, 2),
                Some('<') => (TokenKind::AmperLess, 2),
                Some('|') => (TokenKind::AmperPipe, 2),
                _ => panic!("unexpected character"),
            },

            _ => panic!("unexpected character"),
        };

        let lexeme = self.make_lexeme(self.cursor, self.cursor + length);

        self.cursor += length;

        Token {
            lexeme,
            kind,
        }
    }

    fn scan_number(&mut self) -> Token {
        let mut iter = self.source[self.cursor..].iter();

        let int_part: String = iter.take_while_ref(|&c| c.is_ascii_digit()).collect();

        let dec_part: Option<String> = match iter.next() {
            Some('.') => {
                Some(iter.take_while_ref(|&c| c.is_ascii_digit()).collect())
            }
            _ => None,
        };

        let length = int_part.len() + dec_part.map(|s| 1 + s.len()).unwrap_or(0);
        let lexeme = self.make_lexeme(self.cursor, self.cursor + length);
        let value = lexeme.parse().expect("could not parse number literal");

        self.cursor += lexeme.len();

        Token {
            lexeme,
            kind: TokenKind::Num {
                value,
            },
        }
    }

    fn scan_word(&mut self) -> Token {
        Token {
            lexeme: "".to_owned(),
            kind: TokenKind::Eof,
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.peek_at(0), self.peek_at(1)) {
            (None, _) => None,

            (Some(digit), _) |
            (Some('.'), Some(digit))
            if digit.is_ascii_digit() => Some(self.scan_number()),

            (Some(c), _) if is_word_char(c) => Some(self.scan_word()),

            _ => Some(self.scan_symbol()),
        }
    }
}

fn is_word_char(c: char) -> bool {
    match c {
        '$' | '_' => true,
        _ if c.is_ascii_alphabetic() => true,
        _ => false,
    }
}
