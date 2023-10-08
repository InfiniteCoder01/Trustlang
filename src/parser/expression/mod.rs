use crate::lexer::*;
use orecc_back::ir::*;

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
        matches!(self, Expression::Block(_))
    }
}

// * ------------------------------------- Parse ------------------------------------ * //
pub fn parse(tokens: &mut TokenBuffer) -> Option<Expression> {
    operator::binary(tokens, 0)
}

pub fn expect(tokens: &mut TokenBuffer) -> Option<Expression> {
    if let Some(expression) = parse(tokens) {
        Some(expression)
    } else if let Some(token) = tokens.peek_token() {
        let message = format!("expected expression, got {token}");
        tokens.error(message);
        None
    } else {
        None
    }
}

pub fn literal(tokens: &mut TokenBuffer) -> Option<Expression> {
    if let Some(block) = block::parse(tokens) {
        return Some(Expression::Block(Box::new(block)));
    }
    let token = tokens.next_token()?;
    match token {
        Token::Literal(literal) => Some(Expression::Literal(literal)),
        _ => None,
    }
}

// * ------------------------------------- Tests ------------------------------------ * //
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use orecc_front::Codebase;

//     fn parse<'a>(code: &str, test: &str) -> Expression {
//         let mut codebase = Codebase::new();
//         let file_id = codebase.add(test.to_owned(), code.to_owned());

//         TokenBuffer::new(
//             &mut codebase,
//             codebase.get(file_id).unwrap().source().clone(),
//             file_id,
//         );
//     }

//     #[test]
//     fn binary() {
//         assert_eq!(
//             parse(&mut tokens("2 + 2 * 2", "Classic: 2 + 2 * 2")),
//             Some(Expression::Binary(
//                 Box::new(Expression::Literal(Literal::Int(2))),
//                 BinaryOperation::Add,
//                 Box::new(Expression::Binary(
//                     Box::new(Expression::Literal(Literal::Int(2))),
//                     BinaryOperation::Multiply,
//                     Box::new(Expression::Literal(Literal::Int(2))),
//                 ))
//             ))
//         );
//     }
// }
