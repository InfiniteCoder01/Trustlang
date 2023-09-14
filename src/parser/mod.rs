use crate::lexer::{Keyword, Token, TokenBuffer};
use crate::SpannedError;
use thiserror::Error;

// * --------------------------------- Declarations --------------------------------- * //
#[derive(Clone, Debug)]
pub enum Declaration {
    Function(),
}

pub fn parse_declaration<R: std::io::Read>(
    tokens: &mut TokenBuffer<R>,
) -> Result<Option<Declaration>> {
    if let Some(token) = tokens.peek_token()? {
        Ok(Some(match token.token {
            Token::Ident(_, Some(Keyword::Fn)) => Declaration::Function(),
            _ => return Ok(None),
        }))
    } else {
        Ok(None)
    }
}

// * ------------------------------------ Program ----------------------------------- * //
#[derive(Clone, Debug)]
pub struct Module {
    symbols: Vec<Declaration>,
}

fn parse_module<R: std::io::Read>(tokens: &mut TokenBuffer<R>) -> Result<Module> {
    let mut symbols = Vec::new();
    while let Some(declaration) = parse_declaration(tokens)? {
        symbols.push(declaration);
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
}
