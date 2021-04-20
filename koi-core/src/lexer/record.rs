use crate::token::Token;

use super::raw::RawLexer;

pub struct RecordingLexer {
    wrapped: RawLexer,

    pub is_recording: bool,
    pub record: Vec<(Token, bool)>,

    is_playing_back: bool,

    pub is_new_line: bool,
}

impl RecordingLexer {
    pub fn new(wrapped: RawLexer) -> RecordingLexer {
        RecordingLexer {
            wrapped,

            is_recording: false,
            record: Vec::new(),

            is_playing_back: false,

            is_new_line: true,
        }
    }

    pub fn start_recording(&mut self) {
        self.record = Vec::new();
        self.is_recording = true;
    }

    pub fn stop_recording(&mut self, playback: bool) {
        self.is_recording = false;
        self.is_playing_back = playback;
    }
}

impl Iterator for RecordingLexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_playing_back {
            if self.record.len() > 0 {
                let (token, is_new_line) = self.record.remove(0);
                self.is_new_line = is_new_line;
                return Some(token);
            }

            self.is_playing_back = false;
        }

        let token = self.wrapped.next();

        if self.is_recording && token.is_some() {
            self.record.push((token.clone().unwrap(), self.wrapped.is_new_line));
        }

        self.is_new_line = self.wrapped.is_new_line;

        token
    }
}
