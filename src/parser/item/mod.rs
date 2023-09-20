use crate::lexer::*;
use std::io::Read;

pub mod function;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Item {
    Function(String, super::expression::Block),
}

pub fn parse<R: Read>(tokens: &mut TokenBuffer<R>) -> Option<Item> {
    function::parse(tokens)
}
