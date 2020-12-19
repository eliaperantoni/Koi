use crate::token::{Token, TokenKind};

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

    fn try_scan_symbol(&mut self) -> Option<Token> {
        let kind_length_pair = match self.peek_at(0)? {
            ',' => Some((TokenKind::Comma, 1)),
            '.' => Some((TokenKind::Dot, 1)),
            ':' => Some((TokenKind::Colon, 1)),
            ';' => Some((TokenKind::Semicolon, 1)),

            ' ' => {
                let mut length = 1;
                loop {
                    match self.peek_at(length) {
                        Some(' ') => length += 1,
                        _ => break,
                    }
                }
                Some((TokenKind::Space, length))
            }
            '\n' => Some((TokenKind::Newline, 1)),

            '(' => Some((TokenKind::LeftParen, 1)),
            ')' => Some((TokenKind::RightParen, 1)),
            '[' => Some((TokenKind::LeftBracket, 1)),
            ']' => Some((TokenKind::RightBracket, 1)),
            '{' => Some((TokenKind::LeftBrace, 1)),
            '}' => Some((TokenKind::RightBrace, 1)),

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

                Some(if let Some('=') = self.peek_at(1) {
                    (equal_kind, 2)
                } else {
                    (kind, 1)
                })
            }

            '+' => Some(match self.peek_at(1) {
                Some('+') => (TokenKind::PlusPlus, 2),
                Some('=') => (TokenKind::PlusEqual, 2),
                _ => (TokenKind::Plus, 1),
            }),
            '-' => Some(match self.peek_at(1) {
                Some('-') => (TokenKind::MinusMinus, 2),
                Some('=') => (TokenKind::MinusEqual, 2),
                _ => (TokenKind::Minus, 1),
            }),

            '*' => Some(match self.peek_at(1) {
                Some('=') => (TokenKind::StarEqual, 2),
                Some('>') => (TokenKind::StarGreat, 2),
                Some('<') => (TokenKind::StarLess, 2),
                Some('|') => (TokenKind::StarPipe, 2),
                _ => (TokenKind::Star, 1),
            }),

            '|' => Some(match self.peek_at(1) {
                Some('|') => (TokenKind::PipePipe, 2),
                _ => (TokenKind::Pipe, 1),
            }),

            '&' => match self.peek_at(1) {
                Some('&') => Some((TokenKind::AmperAmper, 2)),
                Some('>') => Some((TokenKind::AmperGreat, 2)),
                Some('<') => Some((TokenKind::AmperLess, 2)),
                Some('|') => Some((TokenKind::AmperPipe, 2)),
                _ => None,
            },

            _ => None,
        };

        if let Some((kind, length)) = kind_length_pair {
            let lexeme = self.make_lexeme(self.cursor, self.cursor + length);

            self.cursor += length;

            Some(Token {
                lexeme,
                kind,
            })
        } else {
            None
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.try_scan_symbol() {
            return Some(token);
        }

        None
    }
}
