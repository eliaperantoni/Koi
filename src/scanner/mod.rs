mod token;
mod string;

#[cfg(test)]
mod test;

pub use token::Token;

pub struct Scanner {
    chars: Vec<char>,
    current: usize,
}

fn is_identifier_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '_'
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            chars: source.chars().collect(),
            current: 0,
        }
    }

    pub fn scan(mut self) -> Vec<Token> {
        let mut tokens = self.scan_tokens(false);
        tokens.push(Token::Eof);
        tokens
    }

    fn scan_tokens(&mut self, in_interpolation: bool) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            self.consume_whitespace();

            if self.is_at_end() {
                return tokens;
            }

            if in_interpolation && self.peek() == '}' {
                return tokens;
            } else if self.peek() == '"' {
                let mut new_tokens = self.scan_string_literal();
                tokens.append(&mut new_tokens);
            } else if self.peek() == '$' && self.remaining() >= 2 && self.peek_n(1) == '(' {
                let mut new_tokens = self.scan_command();
                tokens.append(&mut new_tokens);
            } else if self.peek().is_ascii_digit() || self.peek() == '.' && self.remaining() >= 2 && self.peek_n(1).is_ascii_digit() {
                tokens.push(self.scan_number_literal());
            } else if is_identifier_char(self.peek()) {
                tokens.push(self.scan_word());
            } else {
                tokens.push(self.scan_symbol());
            };
        }
    }

    fn advance(&mut self) -> char {
        self.advance_n(1)
    }

    fn advance_n(&mut self, n: usize) -> char {
        self.current += n - 1;
        let result = self.peek();
        self.current += 1;
        result
    }

    fn peek(&self) -> char {
        self.peek_n(0)
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

    fn remaining(&self) -> usize {
        self.chars.len() - self.current
    }

    // Skips over any whitespace character leaving the scanner over a non-whitespace character or
    // just over the characters array.
    fn consume_whitespace(&mut self) {
        while !self.is_at_end() && self.peek().is_ascii_whitespace() {
            self.advance();
        }
    }

    fn scan_command(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        // Consume initial $(
        self.advance_n(2);

        while self.peek() != ')' {
            self.consume_whitespace();

            let mut new_tokens = if self.peek() == '"' {
                self.scan_string_literal()
            } else {
                self.scan_string(true)
            };

            tokens.append(&mut new_tokens);

            self.consume_whitespace();
        }

        // Consume final )
        self.advance();

        if tokens.len() > 0 {
            match tokens.first_mut().unwrap() {
                Token::String { ref mut begins_cmd, .. } => {
                    *begins_cmd = true;
                }
                _ => {unreachable!();}
            };

            match tokens.last_mut().unwrap() {
                Token::String { ref mut ends_cmd, .. } => {
                    *ends_cmd = true;
                }
                _ => {unreachable!();}
            };
        }

        tokens
    }

    fn scan_word(&mut self) -> Token {
        let start = self.current;

        while !self.is_at_end() && is_identifier_char(self.peek()) {
            self.advance();
        }

        let word_chars = &self.chars[start..self.current];

        match word_chars[0] {
            'i' if word_chars[1] == 'f' => Token::If,

            'w' if check_keyword(&word_chars[1..], "hile") => Token::While,
            'v' if check_keyword(&word_chars[1..], "ar") => Token::Var,
            'r' if check_keyword(&word_chars[1..], "eturn") => Token::Return,
            'b' if check_keyword(&word_chars[1..], "reak") => Token::Break,
            'c' if check_keyword(&word_chars[1..], "ontinue") => Token::Continue,
            't' if check_keyword(&word_chars[1..], "rue") => Token::True,
            'n' if check_keyword(&word_chars[1..], "il") => Token::Nil,

            'f' => {
                match word_chars[1] {
                    'n' => Token::Fn,
                    'o' if word_chars[2] == 'r' => Token::For,
                    'a' if check_keyword(&word_chars[2..], "lse") => Token::False,
                    _ => { Token::Identifier }
                }
            }

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

            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,

            '&' if self.matches('&') => Token::AmperAmper,
            '|' if self.matches('|') => Token::PipePipe,

            '+' => {
                if self.matches('=') { Token::PlusEqual } else if self.matches('+') { Token::PlusPlus } else { Token::Plus }
            }
            '-' => {
                if self.matches('=') { Token::MinusEqual } else if self.matches('-') { Token::MinusMinus } else { Token::Minus }
            }

            '*' => {
                if self.matches('=') { Token::StarEqual } else { Token::Star }
            }
            '/' => {
                if self.matches('=') { Token::SlashEqual } else { Token::Slash }
            }
            '^' => {
                if self.matches('=') { Token::CaretEqual } else { Token::Caret }
            }
            '%' => {
                if self.matches('=') { Token::PercEqual } else { Token::Perc }
            }

            '>' => {
                if self.matches('=') { Token::GreaterEqual } else { Token::Greater }
            }
            '<' => {
                if self.matches('=') { Token::LessEqual } else { Token::Less }
            }

            '!' => {
                if self.matches('=') { Token::BangEqual } else { Token::Bang }
            }
            '=' => {
                if self.matches('=') { Token::EqualEqual } else { Token::Equal }
            }

            _ => { panic!("could not scan symbol") }
        }
    }

    fn scan_number_literal(&mut self) -> Token {
        let start = self.current;

        while !self.is_at_end() && self.peek().is_ascii_digit() {
            self.advance();
        }

        if !self.is_at_end() && self.peek() == '.' {
            self.advance();

            while !self.is_at_end() && self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let lexeme: String = (&self.chars[start..self.current]).into_iter().collect();

        Token::Number {
            value: lexeme.parse().expect("could not parse number literal")
        }
    }
}

// Checks that the char slice source matches the remaining part keyword
fn check_keyword(source: &[char], target: &str) -> bool {
    source == &target.chars().collect::<Vec<_>>()[..]
}
