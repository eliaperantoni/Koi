#[derive(Eq, PartialEq, Debug)]
pub enum Token {
    Identifier,
    IntLiteral,
    FloatLiteral,
    StringLiteral {
        value: String,
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
