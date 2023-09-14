use crate::{lexer::TokenBuffer, SpannedError};
use thiserror::Error;

// * --------------------------------- Declarations --------------------------------- * //
#[derive(Clone, Debug)]
pub enum Declaration {
    Function(),
}

pub fn parse_declaration<R: std::io::Read>(tokens: &mut TokenBuffer<R>) -> Result<Declaration> {
    Ok(Declaration::Function())
}

// * ------------------------------------ Program ----------------------------------- * //
#[derive(Clone, Debug)]
pub struct Module {
    symbols: Vec<Declaration>,
}

fn parse_module<R: std::io::Read>(tokens: &mut TokenBuffer<R>) -> Result<Module> {
    let r#fn = String::from("fn");
    let mut symbols = Vec::new();
    while tokens.peek_token()?.is_some() {
    }
    Ok(Module { symbols })
}

pub struct Function {}

pub type Result<T> = std::result::Result<T, SpannedError<ParserError>>; // TODO: Span wrappr

impl From<SpannedError<crate::lexer::LexerError>> for SpannedError<ParserError> {
    fn from(value: SpannedError<crate::lexer::LexerError>) -> Self {
        Self {
            error: value.error.into(),
            sourcepath: value.sourcepath,
            at: value.at,
        }
    }
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("tokenization failed")]
    Lexer(#[from] crate::lexer::LexerError),
    #[error("unterminated string/character literal")]
    UnterminatedStringLiteral,
    #[error("expected char literal to contain 1 char, but it contains {0} chars")]
    InvalidCharLiteralLength(usize),
    #[error("failed to parse token that starts with {0:?}")]
    DetermineToken(char),
}
