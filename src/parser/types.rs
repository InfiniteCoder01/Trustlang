use super::{ParsingError, Result};
use crate::lexer::*;
use std::io::Read;

#[derive(Clone, Debug)]
pub enum Type {
    Bool,
}

impl Type {
    pub fn parse<R: Read>(tokens: &mut TokenBuffer<R>) -> Result<Self> {
        Ok(
            match tokens
                .next_token()?
                .ok_or(ParsingError::ExpectedTypeGotEof)?
            {
                Token::Ident(_, Some(Keyword::Bool)) => Type::Bool,
                token => return tokens.span(ParsingError::ExpectedType(token)),
            },
        )
    }
}
