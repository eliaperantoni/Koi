pub struct Scanner {
    chars: Vec<char>,
    current: usize,
    start: usize,
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

    fn matches(&mut self, target: &str) -> bool {
        let target_chars = target.chars();

        let mut i = self.current;

        for target_char in target_chars {
            if target_char == self.chars[i] {
                i += 1;
            } else {
                return false;
            }
        };

        self.current = i;
        return true;
    }

    fn is_at_end(&self) -> bool {
        self.current > self.chars.len() - 1
    }
}

impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.start = self.current;

        if self.is_at_end() {
            return None;
        }

        match self.advance() {
            ',' => return Some(Token::Comma),
            ':' => return Some(Token::Colon),
            ';' => return Some(Token::Semicolon),
            '?' => return Some(Token::Question),
            '.' => return Some(Token::Dot),

            '$' if self.matches("(") => return Some(Token::DollarLeftParen),

            '(' => return Some(Token::LeftParen),
            ')' => return Some(Token::RightParen),
            '[' => return Some(Token::LeftBracket),
            ']' => return Some(Token::RightBracket),
            '{' => return Some(Token::LeftBrace),
            '}' => return Some(Token::RightBrace),

            '&' if self.matches("&") => return Some(Token::AmperAmper),
            '|' if self.matches("|") => return Some(Token::PipePipe),

            '+' => {
                return Some(
                    if self.matches("=") { Token::PlusEqual }
                    else if self.matches("+") { Token::PlusPlus }
                    else { Token::Plus }
                );
            },
            '-' => {
                return Some(
                    if self.matches("=") { Token::MinusEqual }
                    else if self.matches("+") { Token::MinusMinus }
                    else { Token::Minus }
                );
            },

            '*' => {
                return Some(if self.matches("=") { Token::StarEqual } else { Token::Star });
            },
            '/' => {
                return Some(if self.matches("=") { Token::SlashEqual } else { Token::Slash });
            },
            '^' => {
                return Some(if self.matches("=") { Token::CaretEqual } else { Token::Caret });
            },
            '%' => {
                return Some(if self.matches("=") { Token::PercEqual } else { Token::Perc });
            },

            '>' => {
                return Some(if self.matches("=") { Token::GreaterEqual } else { Token::Greater });
            },
            '<' => {
                return Some(if self.matches("=") { Token::MinusEqual } else { Token::Minus });
            },

            '!' => {
                return Some(if self.matches("=") { Token::BangEqual } else { Token::Bang });
            },
            '=' => {
                return Some(if self.matches("=") { Token::EqualEqual } else { Token::Equal });
            },

            'f' => {
                if self.matches("or") { return Some(Token::For) }
                else if self.matches("n") { return Some(Token::Fn) }
                else if self.matches("alse") { return Some(Token::False) }
            },

            'w' if self.matches("hile") => { return Some(Token::While) },
            'i' if self.matches("f") => { return Some(Token::If) },
            'e' if self.matches("lse") => { return Some(Token::Else) },
            'r' if self.matches("eturn") => { return Some(Token::Return) },
            'b' if self.matches("reak") => { return Some(Token::Break) },
            'c' if self.matches("ontinue") => { return Some(Token::Continue) },
            't' if self.matches("rue") => { return Some(Token::True) },
            'n' if self.matches("il") => { return Some(Token::Nil) },

            _ => {}
        };

        panic!("unexpected character");
    }
}

#[derive(Debug)]
pub enum Token {
    Identifier,
    IntLiteral,
    FloatLiteral,
    StringLiteral,

    For,
    While,
    If,
    Else,
    Fn,
    Return,
    Break,
    Continue,

    DollarLeftParen,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,

    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    CaretEqual,
    PercEqual,

    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Perc,

    PlusPlus,
    MinusMinus,

    True,
    False,
    Nil,

    AmperAmper,
    PipePipe,

    Comma,
    Semicolon,
    Dot,
    Question,
    Colon,
}
