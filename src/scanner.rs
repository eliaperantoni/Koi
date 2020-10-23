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
            _ => unreachable!()
        };

        unreachable!();
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
    Equal,
    EqualEqual,
    BangEqual,

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
