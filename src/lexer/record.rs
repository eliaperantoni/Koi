use crate::token::{Token, TokenKind};

pub struct RecordingLexer {

}

impl RecordingLexer {
    pub fn new() -> RecordingLexer {

    }
}

impl Iterator for RecordingLexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
