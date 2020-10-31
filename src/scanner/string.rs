use super::*;

impl Scanner {
    pub fn scan_string_literal(&mut self) -> Token {
        // Consume the starting "
        self.advance();

        let mut chars: Vec<char> = Vec::new();

        loop {
            if self.peek() == '"' {
                break;
            }

            if self.matches('\\') {
                let char = self.peek();

                chars.push(match char {
                    't' => '\t',
                    'n' => '\n',
                    'r' => '\r',
                    _ => char,
                });

                // Consume escaped character
                self.advance();

                continue;
            }

            if self.matches('{') {
            }

            chars.push(self.peek());
            self.advance();
        }

        // Consume the closing "
        self.advance();

        let string: String = chars.iter().collect();

        Token::String { value: string, does_interp: false }
    }
}
