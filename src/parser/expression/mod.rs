use super::types::Type;
use crate::lexer::*;
use std::io::Read;

mod operator;

pub use operator::BinaryOperator;

#[derive(Clone, Debug, Default)]
pub enum Expression {
    #[default]
    Unknown,
    Literal(Literal),
    TypeCast(Box<Expression>, Type),
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
}

impl Expression {
    pub fn parse<R: Read>(tokens: &mut TokenBuffer<R>) -> Option<Self> {
        Self::parse_typecast(tokens)
    }

    fn parse_literal<R: Read>(tokens: &mut TokenBuffer<R>) -> Option<Self> {
        let token = tokens.next_token()?;
        match token {
            Token::Literal(literal) => Some(Self::Literal(literal)),
            token => {
                tokens.error(format!("expected expression, got {token}"));
                Some(Expression::Unknown)
            }
        }
    }
}
