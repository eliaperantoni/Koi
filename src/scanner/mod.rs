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

    fn scan_token(&mut self) -> Option<Token> {
        self.consume_whitespace();

        self.start = self.current;

        if self.is_at_end() {
            return None;
        }

        let char = self.peek();

        let token =
            if char == '"' {
                self.scan_string_literal()
            } else if char.is_ascii_digit() || char == '.' && self.remaining() >= 2 && self.peek_n(1).is_ascii_digit() {
                self.scan_number_literal()
            } else if is_identifier_char(char) {
                self.scan_word()
            } else {
                self.scan_symbol()
            };

        Some(token)
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

    fn scan_string_literal(&mut self) -> Token {
        // Consume the starting "
        self.advance();

        loop {
            if self.peek() == '"' {
                break;
            }

            // Consume the backslash (will be filtered later) and the escaped character
            if self.peek() == '\\' {
                self.advance();
                self.advance();
                continue;
            }

            self.advance();
        }

        // Consume the closing "
        self.advance();

        let mut string: String = (&self.chars[self.start+1..self.current-1]).into_iter().collect();
        let string = escape_string(string);

        Token::StringLiteral { value: string }
    }

    fn scan_number_literal(&mut self) -> Token {
        let mut is_float = false;

        while !self.is_at_end() && self.peek().is_ascii_digit() {
            self.advance();
        }

        if !self.is_at_end() && self.peek() == '.' {
            self.advance();

            while !self.is_at_end() && self.peek().is_ascii_digit() {
                self.advance();
            }

            is_float = true;
        }

        let lexeme: String = (&self.chars[self.start..self.current]).into_iter().collect();

        if is_float {
            Token::FloatLiteral {
                value: lexeme.parse().expect("could not parse float literal")
            }
        } else {
            Token::IntLiteral {
                value: lexeme.parse().expect("could not parse int literal")
            }
        }
    }
}

// Checks that the char slice source matches the remaining part keyword
fn check_keyword(source: &[char], target: &str) -> bool {
    source == &target.chars().collect::<Vec<_>>()[..]
}

impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan_token()
    }
}

fn escape_string(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' {
            chars.remove(i);

            chars[i] = match chars[i] {
                't' => '\t',
                'n' => '\n',
                'r' => '\r',
                _ => chars[i],
            }
        }

        i += 1;
    }

    chars.iter().collect()
}
