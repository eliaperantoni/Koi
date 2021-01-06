pub use peek::PeekableLexer as Lexer;
use raw::RawLexer;
use record::RecordingLexer;

mod peek;
mod record;
mod raw;

#[cfg(test)]
mod test;

pub(crate) fn new(source: String) -> Lexer {
    Lexer::new(
        RecordingLexer::new(
            RawLexer::new(source)
        )
    )
}
