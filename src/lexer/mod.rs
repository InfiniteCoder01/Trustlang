use crate::CompilationError;

pub use orecc_front::chacacter_buffer::Cursor;
pub use token_buffer::TokenBuffer;
pub use token_stream::TokenStream;

pub mod token_buffer;
pub mod token_stream;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Ident(String, Option<Keyword>),
    Operator(Operator),
    Literal(Literal),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyword {
    As,
    Fn,

    Bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
    LogicalAnd,
    LogicalOr,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Literal {
    Char(char),
    String(String),
    Bool(bool),
    Int(u128),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(ident, _) => write!(f, "Identifier '{}'", ident),
            Self::Operator(operator) => write!(f, "Operator '{}'", operator),
            Self::Literal(literal) => write!(f, "Literal {}", literal),
        }
    }
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LogicalOr => write!(f, "||"),
            Self::LogicalAnd => write!(f, "&&"),
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Char(characterer) => write!(f, "{:?}", characterer),
            Self::String(string) => write!(f, "{:?}", string),
            Self::Bool(boolean) => write!(f, "{}", boolean),
            Self::Int(integer) => write!(f, "{}", integer),
        }
    }
}

// * ------------------------------------ Errors ------------------------------------ * //
// pub type Result<T> = std::result::Result<T, SpannedError<LexerError>>;

// #[derive(Error, Debug)]
// pub enum LexerError {
//     #[error("reading code from source failed")]
//     Source(#[from] std::io::Error),
//     #[error("unterminated string/character literal")]
//     UnterminatedStringLiteral,
//     #[error("expected char literal to contain 1 char, but it contains {0} chars")]
//     InvalidCharLiteralLength(usize),
//     #[error("failed to parse token that starts with {0:?}")]
//     DetermineToken(char),
// }
