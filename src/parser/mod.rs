use crate::lexer::TokenStream;
use thiserror::Error;

pub trait Parse {
    fn parse<R: std::io::Read>(tokens: &mut TokenStream<R>) -> Result<Self>
    where
        Self: Sized;
}

pub struct Program {
    functions: Vec<Function>,
}

impl Parse for Program {
    fn parse<R: std::io::Read>(tokens: &mut TokenStream<R>) -> Result<Self> {
        let mut functions = Vec::new();
        // while let Some(function) = Function::parse()
        Ok(Self { functions })
    }
}

pub struct Function {}

pub type Result<T> = std::result::Result<T, LexerError>; // TODO: Span wrappr

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("reading code from source failed")]
    Source(#[from] std::io::Error),
    #[error("unterminated string/character literal")]
    UnterminatedStringLiteral,
    #[error("expected char literal to contain 1 char, but it contains {0} chars")]
    InvalidCharLiteralLength(usize),
    #[error("failed to parse token that starts with {0:?}")]
    DetermineToken(char),
}
