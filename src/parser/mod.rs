use crate::SpannedError;
pub use expression::Expression;
use thiserror::Error;

pub mod expression;
pub mod types;

pub type Result<T> = std::result::Result<T, SpannedError<ParsingError>>; // TODO: Span wrappr

impl From<SpannedError<crate::lexer::LexerError>> for SpannedError<ParsingError> {
    fn from(value: SpannedError<crate::lexer::LexerError>) -> Self {
        Self {
            error: value.error.into(),
            sourcepath: value.sourcepath,
            at: value.at,
        }
    }
}

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("{0}")]
    Lexer(#[from] crate::lexer::LexerError),
    #[error("expected expression, got {0}")]
    ExpectedExpression(crate::lexer::Token),
    #[error("expected type, got {0}")]
    ExpectedType(crate::lexer::Token),
    #[error("expected type")]
    ExpectedTypeGotEof,
}
