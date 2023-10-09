use super::{item::function::Function, Crate, Path};
use crate::lexer::*;

pub mod block;
pub mod control_flow;
pub mod value;
// pub mod operator;

pub use value::{Target, Value, Variable};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Expression {
    // Binary(Target, BinaryOperation, Box<Value>, Box<Value>),
    Return(Value),
}

// * ------------------------------------- Parse ------------------------------------ * //
pub fn parse(
    tokens: &mut TokenBuffer,
    crate_: &mut Crate,
    function: &mut Function,
    path: &Path,
) -> Option<Value> {
    control_flow::parse(tokens, crate_, function, path)
}

pub fn expect(
    tokens: &mut TokenBuffer,
    crate_: &mut Crate,
    function: &mut Function,
    path: &Path,
) -> Option<Value> {
    match parse(tokens, crate_, function, path) {
        Some(value) => Some(value),
        _ => {
            tokens.emit_expected("expression");
            None
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Expression::Return(value) => write!(f, "return {value}"),
        }
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
