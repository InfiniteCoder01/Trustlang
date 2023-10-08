mod token_buffer;
mod token_stream;

pub use token_buffer::TokenBuffer;
pub type Span = std::ops::Range<usize>;
pub type CrossfileSpan = (usize, Span);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Ident(String, Option<Keyword>),
    Operator(Operator),
    Literal(Literal),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyword {
    Fn,
    Mod,

    As,

    Bool,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Modulo,

    Ampersand,
    Bar,
    Carrot,

    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    LogicalAnd,
    LogicalOr,
    ShiftLeft,
    ShiftRight,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Literal {
    Char(char),
    String(String),
    Bool(bool),
    Int(u128),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(ident, _) => write!(f, "Identifier '{ident}'"),
            Self::Operator(operator) => write!(f, "Operator '{operator}'"),
            Self::Literal(literal) => write!(f, "Literal {literal}"),
        }
    }
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Modulo => write!(f, "%"),

            Self::Ampersand => write!(f, "&"),
            Self::Bar => write!(f, "|"),
            Self::Carrot => write!(f, "^"),

            Self::Semicolon => write!(f, ";"),

            Self::LParen => write!(f, "("),
            Self::RParen => write!(f, ")"),
            Self::LBrace => write!(f, "{{"),
            Self::RBrace => write!(f, "}}"),

            Self::LogicalAnd => write!(f, "&&"),
            Self::LogicalOr => write!(f, "||"),
            Self::ShiftLeft => write!(f, "<<"),
            Self::ShiftRight => write!(f, ">>"),
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Char(characterer) => write!(f, "{characterer:?}"),
            Self::String(string) => write!(f, "{string:?}"),
            Self::Bool(boolean) => write!(f, "{boolean}"),
            Self::Int(integer) => write!(f, "{integer}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
}

impl SpannedToken {
    pub fn new(token: Token, span: Span) -> Self {
        Self { token, span }
    }
}
