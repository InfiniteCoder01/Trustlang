use logos::{Lexer, Logos};

fn int_literal(value: &str, radix: u32) -> Option<Literal> {
    Some(Literal::Int(u128::from_str_radix(value, radix).ok()?))
}

/// Lexical token - an identifier, an operator, a literal, etc.
#[derive(Logos, Clone, Debug, PartialEq, Eq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    /// A literal - Boolean, char, string, integer, etc.
    #[token("true", |_| Literal::Bool(true))]
    #[token("false", |_| Literal::Bool(false))]
    #[regex("0x[0-9a-fA-F]+", |lex| int_literal(&lex.slice()[2..], 16))]
    #[regex("0b[0-1]+", |lex| int_literal(&lex.slice()[2..], 2))]
    #[regex("0o[0-7]+", |lex| int_literal(&lex.slice()[2..], 8))]
    #[regex("[0-9]+", |lex| lex.slice().parse().ok().map(Literal::Int))]
    Literal(Literal),

    // TODO: Once Logos supports multiple field variants, this can be simplified to hold an ident and an optional keyword
    /// An identifier. Can be a keyword. If it's a keyword, still can be used as a name
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", Ident::parse)]
    Ident(Ident),

    /// Operator token - Plus, Semicolon, Paren, etc.
    #[token("+", |_| Operator::Plus)]
    #[token("-", |_| Operator::Minus)]
    #[token("*", |_| Operator::Star)]
    #[token("/", |_| Operator::Slash)]
    #[token("%", |_| Operator::Modulo)]
    #[token("&", |_| Operator::Ampersand)]
    #[token("|", |_| Operator::Bar)]
    #[token("^", |_| Operator::Carrot)]
    #[token(";", |_| Operator::Semicolon)]
    #[token("(", |_| Operator::LParen)]
    #[token(")", |_| Operator::RParen)]
    #[token("{", |_| Operator::LBrace)]
    #[token("}", |_| Operator::RBrace)]
    #[token("&&", |_| Operator::LogicalAnd)]
    #[token("||", |_| Operator::LogicalOr)]
    #[token("<<", |_| Operator::ShiftLeft)]
    #[token(">>", |_| Operator::ShiftRight)]
    Operator(Operator),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(ident) => write!(f, "identifier '{ident}'"),
            Self::Operator(operator) => write!(f, "operator '{operator}'"),
            Self::Literal(literal) => write!(f, "literal {literal}"),
        }
    }
}

// * ---------------------------------- Identifier ---------------------------------- * //
/// An identifier. Can be a keyword. If it's a keyword, still can be used as a name
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ident {
    /// Declare a function
    Fn,
    /// Declare a module
    Mod,

    /// Return a value
    Return,

    /// Typecast
    As,

    /// Boolean type
    Bool,

    /// Just an identifier
    Other(String),
}

impl Ident {
    fn parse(lex: &mut Lexer<Token>) -> Option<Self> {
        Some(match lex.slice() {
            "fn" => Self::Fn,
            "mod" => Self::Mod,

            "return" => Self::Return,
            "as" => Self::As,

            "bool" => Self::Bool,

            ident => Self::Other(ident.to_owned()),
        })
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fn => write!(f, "fn"),
            Self::Mod => write!(f, "mod"),

            Self::Return => write!(f, "return"),
            Self::As => write!(f, "as"),

            Self::Bool => write!(f, "bool"),

            Self::Other(ident) => write!(f, "{ident}"),
        }
    }
}

// * ----------------------------------- Operator ----------------------------------- * //
/// Operator token - Plus, Semicolon, Paren, etc.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Operator {
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Star,
    /// /
    Slash,
    /// %
    Modulo,

    /// &
    Ampersand,
    /// |
    Bar,
    /// ^
    Carrot,

    /// ;
    Semicolon,

    /// (
    LParen,
    /// )
    RParen,
    /// {
    LBrace,
    /// }
    RBrace,

    /// &&
    LogicalAnd,
    /// ||
    LogicalOr,
    /// <<
    ShiftLeft,
    /// >>
    ShiftRight,
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

// * ------------------------------------ Literal ----------------------------------- * //
/// A literal - Boolean, char, string, integer, etc.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Literal {
    /// Character
    Char(char),
    /// String
    String(String),
    /// Boolean
    Bool(bool),
    /// Integer
    Int(u128),
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
