use crate::token::{Token, TokenKind};
use itertools::Itertools;

#[cfg(test)]
mod test;

pub struct Lexer {
    source: Vec<char>,
    cursor: usize,

    interp_count: u8,
    braces_count: u8,

    buffer: Vec<Token>,

    sent_eof: bool,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source: source.chars().collect(),
            cursor: 0,

            interp_count: 0,
            braces_count: 0,

            buffer: Vec::new(),

            sent_eof: false,
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

            '{' => {
                self.braces_count += 1;
                (TokenKind::LeftBrace, 1)
            }
            '}' => {
                self.braces_count -= 1;
                (TokenKind::RightBrace, 1)
            }

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
            kind: TokenKind::Num(value),
        }
    }

    fn scan_word(&mut self) -> Token {
        let mut iter = self.source[self.cursor..].iter();

        let word: String = iter.take_while_ref(|&&c| can_start_word(c) || c.is_ascii_digit()).collect();

        let kw_kind = match word.as_ref() {
            "for" => Some(TokenKind::For),
            "while" => Some(TokenKind::While),
            "if" => Some(TokenKind::If),
            "else" => Some(TokenKind::Else),
            "fn" => Some(TokenKind::Fn),
            "return" => Some(TokenKind::Return),
            "break" => Some(TokenKind::Break),
            "continue" => Some(TokenKind::Continue),
            "var" => Some(TokenKind::Var),
            "exp" => Some(TokenKind::Exp),
            "true" => Some(TokenKind::True),
            "false" => Some(TokenKind::False),
            "nil" => Some(TokenKind::Nil),
            _ => None,
        };

        self.cursor += word.len();

        if let Some(kind) = kw_kind {
            Token {
                lexeme: word.clone(),
                kind,
            }
        } else {
            Token {
                lexeme: word.clone(),
                kind: TokenKind::Identifier(word.clone()),
            }
        }
    }

    fn scan_string(&mut self) -> Token {
        // A string literal is scanned in one go. The first token is returned, the rest is saved in
        // a buffer and tokens are returned in the next calls to `next`
        let mut tokens = Vec::new();

        // Either ' or "
        // Safe to unwrap because the lexer calls this method when the current char is ' or " so there
        // is at least one character
        let delimiter = self.peek_at(0).unwrap();

        let mut lexeme_start = self.cursor;

        // Consume delimiter
        self.cursor += 1;

        // Piece of string between delimiters and/or braces
        let mut literal_piece = String::new();

        loop {
            let ch = if let Some(ch) = self.peek_at(0) {
                ch
            } else {
                panic!("unterminated string");
            };

            self.cursor += 1;

            if ch == delimiter {
                tokens.push(Token {
                    lexeme: self.make_lexeme(lexeme_start, self.cursor),
                    kind: TokenKind::String {
                        value: literal_piece.clone(),
                        does_interp: false,
                    },
                });
                break;
            }

            if ch == '{' {
                tokens.push(Token {
                    lexeme: self.make_lexeme(lexeme_start, self.cursor),
                    kind: TokenKind::String {
                        value: literal_piece.clone(),
                        does_interp: true,
                    },
                });

                self.interp_count += 1;
                tokens.append(&mut self.collect::<Vec<Token>>());
                self.interp_count -= 1;

                match self.peek_at(0) {
                    Some('}') => (),
                    _ => panic!("expected closing brace at end of interpolated expression")
                }

                lexeme_start = self.cursor;
                literal_piece = String::new();

                self.cursor += 1;

                continue;
            }

            if ch == '\\' {
                let ch = match self.peek_at(0) {
                    Some('n') => '\n',
                    Some('t') => '\t',
                    Some('r') => '\r',
                    Some('\\') => '\\',
                    Some(_) => panic!("unexpected escape character"),
                    None => panic!("unterminated string"),
                };

                literal_piece.push(ch);
                self.cursor += 1;
            } else {
                literal_piece.push(ch);
            }
        }

        let first = tokens.remove(0);
        self.buffer.append(&mut tokens);
        first
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.buffer.is_empty() {
            return Some(self.buffer.remove(0));
        }

        match (self.peek_at(0), self.peek_at(1)) {
            (None, _) => if self.sent_eof {
                None
            } else {
                self.sent_eof = true;
                Some(Token {
                    lexeme: "".to_owned(),
                    kind: TokenKind::Eof,
                })
            },

            (Some('}'), _) if self.interp_count > 0 && self.braces_count == 0 => None,

            (Some(digit), _) | (Some('.'), Some(digit)) if digit.is_ascii_digit() => Some(self.scan_number()),
            (Some('"'), _) | (Some('\''), _) => Some(self.scan_string()),

            (Some(c), _) if can_start_word(c) => Some(self.scan_word()),

            _ => Some(self.scan_symbol()),
        }
    }
}

fn can_start_word(c: char) -> bool {
    ['$', '_'].contains(&c) || c.is_ascii_alphabetic()
}
