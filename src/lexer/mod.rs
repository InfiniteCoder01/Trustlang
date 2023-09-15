use crate::SpannedError;
use thiserror::Error;

pub use chacacter_buffer::Cursor;
pub use token_buffer::TokenBuffer;
pub use token_stream::TokenStream;

pub mod chacacter_buffer;
pub mod token_buffer;
pub mod token_stream;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Ident(String, Option<Keyword>),
    Literal(Literal),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyword {
    As,
    Fn,

    Bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Literal {
    Char(char),
    String(String),
    Bool(bool),
    Int(u32),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Ident(ident, _) => write!(f, "Identifier '{}'", ident),
            Token::Literal(literal) => write!(f, "Literal {}", literal),
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Char(characterer) => write!(f, "{:?}", characterer),
            Literal::String(string) => write!(f, "{:?}", string),
            Literal::Bool(boolean) => write!(f, "{}", boolean),
            Literal::Int(integer) => write!(f, "{}", integer),
        }
    }
}

// * ---------------------------------- Check chars --------------------------------- * //
mod chars {
    use lazy_regex::regex;

    #[cfg(feature = "xid")]
    pub(super) fn is_ident_start(char: char) -> bool {
        unicode_ident::is_xid_start(char) || char == '_'
    }

    #[cfg(feature = "xid")]
    pub(super) fn is_ident_continue(char: char) -> bool {
        unicode_ident::is_xid_continue(char)
    }

    #[cfg(not(feature = "xid"))]
    pub(super) fn is_ident_start(char: char) -> bool {
        let regex = regex!("[a-zA-Z_]");
        regex.is_match(&char.to_string())
    }

    #[cfg(not(feature = "xid"))]
    pub(super) fn is_ident_continue(char: char) -> bool {
        let regex = regex!("[a-zA-Z0-9_]");
        regex.is_match(&char.to_string())
    }

    pub(super) fn is_whitespace(char: char) -> bool {
        char.is_whitespace()
    }
}

// * ------------------------------------ Errors ------------------------------------ * //
pub type Result<T> = std::result::Result<T, SpannedError<LexerError>>;

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
