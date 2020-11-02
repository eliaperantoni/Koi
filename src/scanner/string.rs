use super::*;

impl Scanner {
    pub fn scan_string_literal(&mut self) {
        // Consume initial "
        self.advance();

        self.scan_string(false);

        // Consume final "
        self.advance();
    }

    pub fn scan_string(&mut self, in_command: bool) {
        loop {
            let mut chars: Vec<char> = Vec::new();

            loop {
                if self.peek() == '"' || self.peek() == '{' {
                    break;
                }

                if in_command && (self.peek() == ' ' || self.peek() == ')') {
                    break;
                }

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
            self.tokens.push(Token::String {
                value: string,
                does_interp: self.peek() == '{',
                begins_cmd: false,
                ends_cmd: false,
            });

            if self.peek() == '{' {
                self.advance();
                self.scan_tokens(true);
                self.advance();
            } else if self.peek() == '"' {
                break;
            }
        }
    }
}
