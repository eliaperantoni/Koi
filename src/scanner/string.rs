use super::*;

impl Scanner {
    pub fn scan_string_literal(&mut self) {
        // Consume initial "
        self.advance();

        loop {
            let mut chars: Vec<char> = Vec::new();

            while self.peek() != '"' && self.peek() != '{' {
                if self.peek() == '\\' {
                    self.advance();
                    
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

                chars.push(self.peek());
                self.advance();
            }

            let string: String = chars.iter().collect();
            self.tokens.push(Token::String { value: string, does_interp: self.peek() == '{' });

            if self.peek() == '{' {
                self.advance();
                self.scan_tokens(true);
                self.advance();
            } else if self.peek() == '"' {
                break;
            }
        }

        // Consume final "
        self.advance();
    }
}
