#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    Identifier(String),
    Num(f64),
    String {
        value: String,
        does_interp: bool,
    },

    Import,
    As,
    For,
    In,
    While,
    If,
    Else,
    Fn,
    Return,
    Break,
    Continue,
    Let,
    Exp,

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

    Great,
    GreatEqual,
    Less,
    LessEqual,

    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Perc,

    True,
    False,
    Nil,

    AmperAmper,
    PipePipe,

    Arrow,

    Comma,
    DotDot,
    Dot,
    Colon,

    Dollar,
    DollarLeftParen,

    // Commands related stuff ...

    Pipe,
    StarPipe,
    AmperPipe,

    StarGreat,
    AmperGreat,

    GreatGreat,
    StarGreatGreat,
    AmperGreatGreat,

    Semicolon,

    // ... until here

    Space,
    Newline,

    UnknownChar(char),
}
