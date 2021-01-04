use crate::token::{Token, TokenKind};

use super::raw::RawLexer;

pub struct PeekableLexer {
    raw: RawLexer,
    peeked: Option<Token>,
    pub is_new_line: bool,
}

impl PeekableLexer {
    pub fn new(raw: RawLexer) -> PeekableLexer {
        PeekableLexer {
            raw,
            peeked: None,
            is_new_line: true,
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        if self.peeked.is_none() {
            self.peeked = self.raw.next();
        }

        self.peeked.as_ref()
    }
}

impl Iterator for PeekableLexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = if self.peeked.is_some() {
            self.peeked.take()
        } else {
            self.raw.next()
        };

        self.is_new_line = self.raw.is_new_line;
        token
    }
}
