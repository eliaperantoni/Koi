use crate::token::{Token, TokenKind};

use super::record::RecordingLexer;

pub struct PeekableLexer {
    wrapped: RecordingLexer,

    peeked: Option<Token>,

    pub is_new_line: bool,
}

impl PeekableLexer {
    pub fn new(wrapped: RecordingLexer) -> PeekableLexer {
        PeekableLexer {
            wrapped,

            peeked: None,

            is_new_line: true,
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        if self.peeked.is_none() {
            self.peeked = self.wrapped.next();

            if self.wrapped.is_recording {
                self.wrapped.record.remove(self.wrapped.record.len() - 1);
            }
        }

        self.peeked.as_ref()
    }

    pub fn consume_whitespace(&mut self, newlines: bool) {
        while match self.peek() {
            Some(Token { kind: TokenKind::Space, .. }) => true,
            Some(Token { kind: TokenKind::Newline, .. }) if newlines => true,
            _ => false,
        } {
            self.next();
        }
    }
}

impl Iterator for PeekableLexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = if self.peeked.is_some() {
            if self.wrapped.is_recording {
                self.wrapped.record.insert(0, (self.peeked.clone().unwrap(), false));
            }

            self.peeked.take()
        } else {
            self.wrapped.next()
        };

        self.is_new_line = self.wrapped.is_new_line;
        token
    }
}

impl PeekableLexer {
    pub fn start_recording(&mut self) {
        self.wrapped.start_recording();
    }

    pub fn stop_recording(&mut self, playback: bool) {
        self.wrapped.stop_recording(playback);
    }
}
