use itertools::__std_iter::Peekable;
use crate::lexer::Lexer;

mod expr;
mod stmt;

pub struct Parser {
    lexer: Peekable<Lexer>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer: lexer.peekable(),
        }
    }
}
