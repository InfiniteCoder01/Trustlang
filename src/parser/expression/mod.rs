pub use super::types::Type;
use crate::lexer::*;
use orecc_back::ast::*;
use std::io::Read;

// pub mod block;
pub mod operator;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Expression {
    Literal(Literal),
    Binary(Box<Expression>, BinaryOperation, Box<Expression>),
}

// * ------------------------------------ Parsers ----------------------------------- * //
pub fn parse<R: Read>(tokens: &mut TokenBuffer<R>) -> Option<Expression> {
    operator::binary(tokens, 0)
}

pub fn literal<R: Read>(tokens: &mut TokenBuffer<R>) -> Option<Expression> {
    let token = tokens.next_token()?;
    match token {
        Token::Literal(literal) => Some(Expression::Literal(literal)),
        token => {
            tokens.error(format!("expected expression, got {token}"));
            None
        }
    }
}

// * ------------------------------------- Tests ------------------------------------ * //
#[cfg(test)]
mod tests {
    use super::*;

    fn tokens<'a>(code: &'a str, test: &str) -> TokenBuffer<std::io::Cursor<&'a str>> {
        TokenBuffer::new(TokenStream::new(std::io::Cursor::new(code), Some(test)))
    }

    #[test]
    fn binary() {
        assert_eq!(
            parse(&mut tokens("2 + 2 * 2", "Classic: 2 + 2 * 2")),
            Some(Expression::Binary(
                Box::new(Expression::Literal(Literal::Int(2))),
                BinaryOperation::Add,
                Box::new(Expression::Binary(
                    Box::new(Expression::Literal(Literal::Int(2))),
                    BinaryOperation::Multiply,
                    Box::new(Expression::Literal(Literal::Int(2))),
                ))
            ))
        );
    }
}
