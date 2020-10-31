use super::*;

impl Scanner {
    pub fn scan_string_literal(&mut self) -> Token {
        let mut does_interp = false;

        let mut chars: Vec<char> = Vec::new();

        loop {
            if self.peek() == '"' {
                // Consume the closing "
                self.advance();
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
                does_interp = true;
                self.interp_count += 1;
                break;
            }

            chars.push(self.peek());
            self.advance();
        }

        let string: String = chars.iter().collect();

        Token::String { value: string, does_interp }
    }
}
