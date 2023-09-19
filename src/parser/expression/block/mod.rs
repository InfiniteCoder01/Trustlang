use crate::lexer::*;
use orecc_back::Backend;
use std::io::Read;

pub fn parse<R: Read, B: Backend>(
    tokens: &mut TokenBuffer<R>,
    backend: &mut B,
) -> Option<B::DataType> {
    //
}
