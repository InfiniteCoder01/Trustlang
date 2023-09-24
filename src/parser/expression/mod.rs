pub use super::types::Type;
use crate::lexer::*;
use orecc_back::ir::*;
use std::io::Read;

pub mod block;
pub mod operator;

pub use block::Block;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Expression {
    Tuple(Vec<Expression>),
    Block(Box<block::Block>),

    Literal(Literal),
    Binary(Box<Expression>, BinaryOperation, Box<Expression>),
}

impl Expression {
    pub fn is_block(&self) -> bool {
        match self {
            Expression::Block(_) => true,
            _ => false,
        }
    }
}

// * ------------------------------------- Parse ------------------------------------ * //
pub fn parse<R: Read>(tokens: &mut TokenBuffer<R>) -> Option<Expression> {
    operator::binary(tokens, 0)
}

pub fn literal<R: Read>(tokens: &mut TokenBuffer<R>) -> Option<Expression> {
    if let Some(block) = block::parse(tokens) {
        return Some(Expression::Block(Box::new(block)));
    }
    let token = tokens.next_token()?;
    match token {
        Token::Literal(literal) => Some(Expression::Literal(literal)),
        token => {
            tokens.error(format!("expected expression, got {token}"));
            None
        }
    }
}

// * ------------------------------------- Build ------------------------------------ * //
pub enum BuiltValue {
    Never,
    Unit,
    Data { data: Value },
}

impl BuiltValue {
    pub fn data(data: Value) -> Self {
        Self::Data { data }
    }

    pub fn as_data(self) -> Option<Value> {
        match self {
            BuiltValue::Never => None,
            BuiltValue::Unit => None,
            BuiltValue::Data { data } => Some(data),
        }
    }
}

impl Expression {
    pub fn build(self, module: &Module, function: &mut Function) -> BuiltValue {
        match self {
            Expression::Tuple(values) => {
                if values.is_empty() {
                    BuiltValue::Unit
                } else {
                    todo!()
                }
            }
            Expression::Block(_) => todo!(),
            Expression::Literal(literal) => match literal {
                Literal::Char(_) => todo!(),
                Literal::String(_) => todo!(),
                Literal::Bool(_) => todo!(),
                Literal::Int(value) => BuiltValue::Data {
                    data: Value::Unsigned(value),
                },
            },
            Expression::Binary(lhs, op, rhs) => {
                let lhs = lhs.build(module, function);
                let rhs = rhs.build(module, function);
                match (lhs.as_data(), rhs.as_data()) {
                    (Some(lhs), Some(rhs)) => BuiltValue::data(function.binary_op(op, lhs, rhs)),
                    _ => todo!("Type checking"),
                }
            }
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
