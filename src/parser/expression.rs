use super::{ParsingError, Result};
use crate::lexer::*;
use std::io::Read;

#[derive(Clone, Debug)]
pub enum Expression {
    Literal(Literal),
}

impl Expression {
    pub fn parse<R: Read>(tokens: &mut TokenBuffer<R>) -> Result<Option<Self>> {
        Self::literal(tokens)
    }

    fn literal<R: Read>(tokens: &mut TokenBuffer<R>) -> Result<Option<Self>> {
        if let Some(token) = tokens.next_token()? {
            match token {
                Token::Literal(literal) => Ok(Some(Self::Literal(literal))),
                token => tokens.span(ParsingError::ExpectedExpression(token)),
            }
        } else {
            Ok(None)
        }
    }
}
