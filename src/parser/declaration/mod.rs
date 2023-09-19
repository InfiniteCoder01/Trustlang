use crate::lexer::*;
use orecc_back::Backend;
use std::io::Read;

pub mod function;

pub fn parse<R: Read, B: Backend>(tokens: &mut TokenBuffer<R>, backend: &mut B) -> bool {
    function::parse(tokens, backend)
}
