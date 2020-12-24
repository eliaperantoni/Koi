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

    For,
    While,
    If,
    Else,
    Fn,
    Return,
    Break,
    Continue,
    Var,
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

    PlusPlus,
    MinusMinus,

    True,
    False,
    Nil,

    AmperAmper,
    PipePipe,

    Comma,
    Dot,
    Colon,

    // Commands related stuff ...

    Pipe,
    StarPipe,
    AmperPipe,

    StarGreat,
    AmperGreat,

    StarLess,
    AmperLess,

    Semicolon,

    // ... until here

    Space,
    Newline,

    Eof,
}
