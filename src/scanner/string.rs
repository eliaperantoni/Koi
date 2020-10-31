use super::*;

impl Scanner {
    pub fn scan_string_literal(&mut self) -> Token {
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

/*
    "ciao {num} ciao"
    "ciao " + num + " ciao"
 */
