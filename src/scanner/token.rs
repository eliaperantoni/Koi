#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Identifier,
    Num {
        value: f64,
    },
    String {
        value: String,
        does_interp: bool,
        begins_cmd: bool,
        ends_cmd: bool,
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

    Eof,
}

impl Token {
    pub fn is_infix_op(&self) -> bool {
        use Token::*;

        match self {
            BangEqual |
            Equal |
            EqualEqual |

            PlusEqual |
            MinusEqual |
            StarEqual |
            SlashEqual |
            CaretEqual |
            PercEqual |

            Greater |
            GreaterEqual |
            Less |
            LessEqual |

            Plus |
            Minus |
            Star |
            Slash |
            Caret |
            Perc |

            AmperAmper |
            PipePipe => true,

            _ => false,
        }
    }
}
