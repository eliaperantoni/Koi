use super::*;

impl Scanner {
    pub fn scan_string_literal(&mut self) -> Vec<Token> {
        // Consume initial "
        self.advance();

        let tokens = self.scan_string(false);

        // Consume final "
        self.advance();

        tokens
    }

    // Scans a string literal without the quotes. If in_cmd is true then the scanning will stop when
    // encountering a whitespace char or a right parenthesis
    pub fn scan_string(&mut self, in_cmd: bool) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let mut chars: Vec<char> = Vec::new();

            loop {
                if self.peek() == '"' || self.peek() == '{' {
                    break;
                }

                if in_cmd && (self.peek().is_ascii_whitespace() || self.peek() == ')') {
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
            tokens.push(Token::String {
                value: string,
                does_interp: self.peek() == '{',
                begins_cmd: false,
                ends_cmd: false,
            });

            if self.peek() == '{' {
                self.advance();

                tokens.append(&mut self.scan_tokens(true));

                self.advance();
            } else if self.peek() == '"' {
                break;
            } else if in_cmd && (self.peek().is_ascii_whitespace() || self.peek() == ')') {
                break;
            }
        }

        tokens
    }
}
