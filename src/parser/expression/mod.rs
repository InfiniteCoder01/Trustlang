pub use super::types::Type;
use crate::lexer::*;
use orecc_back::Backend;
use std::io::Read;

pub mod operator;

pub fn parse<R: Read, B: Backend>(
    tokens: &mut TokenBuffer<R>,
    backend: &mut B,
) -> Option<B::DataType> {
    operator::parse_logical_or(tokens, backend)
}

pub fn parse_literal<R: Read, B: Backend>(
    tokens: &mut TokenBuffer<R>,
    backend: &mut B,
) -> Option<B::DataType> {
    let token = tokens.next_token()?;
    match token {
        Token::Literal(literal) => Some(match literal {
            Literal::Char(_) => todo!(),
            Literal::String(_) => todo!(),
            Literal::Bool(_) => todo!(),
            Literal::Int(integer) => backend.unsigned_autosize(integer),
        }),
        token => {
            tokens.error(format!("expected expression, got {token}"));
            None
        }
    }
}
