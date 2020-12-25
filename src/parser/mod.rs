use itertools::__std_iter::Peekable;
use crate::lexer::Lexer;

mod expr;
mod stmt;

#[cfg(test)]
mod test;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer,
        }
    }
}
