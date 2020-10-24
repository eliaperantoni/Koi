mod token;

#[cfg(test)]
mod test;

pub use token::Token;

pub struct Scanner {
    chars: Vec<char>,
    current: usize,
    start: usize,
}

fn is_identifier_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '_'
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            chars: source.chars().collect(),
            current: 0,
            start: 0,
        }
    }

    fn advance(&mut self) -> char {
        let result = self.peek();
        self.current += 1;
        result
    }

    fn peek(&self) -> char {
        self.chars[self.current]
    }

    fn peek_n(&self, n: usize) -> char {
        self.chars[self.current + n]
    }

    fn matches(&mut self, c: char) -> bool {
        if self.peek() == c {
            self.advance();
            true
        } else {
            false
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn consume_whitespace(&mut self) {
        const WHITESPACE_CHARS: [char; 4] = [' ', '\t', '\n', '\r'];

        while !self.is_at_end() && WHITESPACE_CHARS.contains(&self.peek()) {
            self.advance();
        }
    }

    fn scan_token(&mut self) -> Option<Token> {
        self.start = self.current;

        self.consume_whitespace();

        if self.is_at_end() {
            return None;
        }

        Some(if is_identifier_char(self.peek()) {
            self.scan_word()
        } else {
            self.scan_symbol()
        })
    }

    fn scan_word(&mut self) -> Token {
        while !self.is_at_end() && is_identifier_char(self.peek()) {
            self.advance();
        }

        let word_chars = &self.chars[self.start..self.current];

        match word_chars[0] {
            'i' if word_chars[1] == 'f' => Token::If,

            'w' if check_keyword(&word_chars[1..], "hile") => Token::While,
            'v' if check_keyword(&word_chars[1..], "ar") => Token::Var,
            'r' if check_keyword(&word_chars[1..], "eturn") => Token::Return,
            'b' if check_keyword(&word_chars[1..], "eturn") => Token::Break,
            'c' if check_keyword(&word_chars[1..], "eturn") => Token::Continue,
            't' if check_keyword(&word_chars[1..], "rue") => Token::True,
            'n' if check_keyword(&word_chars[1..], "il") => Token::Nil,

            'f' => {
                match word_chars[1] {
                    'n' => Token::Fn,
                    'o' if word_chars[2] == 'r' => Token::For,
                    'a' if check_keyword(&word_chars[2..], "lse") => Token::While,
                    _ => { Token::Identifier }
                }
            },

            'e' => {
                match word_chars[1] {
                    'x' if word_chars[2] == 'p' => Token::Exp,
                    'l' if check_keyword(&word_chars[2..], "se") => Token::Else,
                    _ => { Token::Identifier }
                }
            }

            _ => { Token::Identifier }
        }
    }

    fn scan_symbol(&mut self) -> Token {
        match self.advance() {
            ',' => Token::Comma,
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            '?' => Token::Question,
            '.' => Token::Dot,

            '$' if self.matches('(') => Token::DollarLeftParen,

            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,

            '&' if self.matches('&') => Token::AmperAmper,
            '|' if self.matches('|') => Token::PipePipe,

            '+' => {
                if self.matches('=') { Token::PlusEqual }
                else if self.matches('+') { Token::PlusPlus }
                else { Token::Plus }
            }
            '-' => {
                if self.matches('=') { Token::MinusEqual }
                else if self.matches('+') { Token::MinusMinus }
                else { Token::Minus }
            }

            '*' => {
                if self.matches('=') { Token::StarEqual }
                else { Token::Star }
            }
            '/' => {
                if self.matches('=') { Token::SlashEqual }
                else { Token::Slash }
            }
            '^' => {
                if self.matches('=') { Token::CaretEqual }
                else { Token::Caret }
            }
            '%' => {
                if self.matches('=') { Token::PercEqual }
                else { Token::Perc }
            }

            '>' => {
                if self.matches('=') { Token::GreaterEqual }
                else { Token::Greater }
            }
            '<' => {
                if self.matches('=') { Token::LessEqual }
                else { Token::Less }
            }

            '!' => {
                if self.matches('=') { Token::BangEqual }
                else { Token::Bang }
            }
            '=' => {
                if self.matches('=') { Token::EqualEqual }
                else { Token::Equal }
            }

            _ => { panic!("could not scan symbol") }
        }
    }
}

fn check_keyword(source: &[char], target: &str) -> bool {
    source == &target.chars().collect::<Vec<_>>()[..]
}

impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan_token()
    }
}