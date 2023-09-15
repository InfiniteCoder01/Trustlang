use super::types::Type;
use super::{ParsingError, Result};
use crate::lexer::*;
use std::io::Read;

#[derive(Clone, Debug)]
pub enum Expression {
    Literal(Literal),
    TypeCast(Box<Expression>, Type),
}

impl Expression {
    pub fn parse<R: Read>(tokens: &mut TokenBuffer<R>) -> Result<Option<Self>> {
        Self::binary_expression(tokens, 0)
    }

    fn binary_expression<R: Read>(
        tokens: &mut TokenBuffer<R>,
        precedance: u32,
    ) -> Result<Option<Self>> {
        let lhs = match (precedance as i32) < 0 {
            true => Self::binary_expression(tokens, precedance + 1)?,
            false => Self::literal(tokens)?,
        };
        let lhs = match lhs {
            Some(lhs) => lhs,
            None => return Ok(None),
        };

        match precedance {
            0 => {
                if tokens.match_keyword(Keyword::As)? {
                    return Ok(Some(Expression::TypeCast(
                        Box::new(lhs),
                        super::types::Type::parse(tokens)?,
                    )));
                }
            }
            precedance => panic!(
                "Internal error: invalid binary operator precedance: {}",
                precedance
            ),
        }
        Ok(Some(lhs))
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
