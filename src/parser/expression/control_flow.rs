use super::Value;
use crate::lexer::*;
use crate::parser::{item::function::Function, Crate, Path};

pub fn parse(
    tokens: &mut TokenBuffer,
    crate_: &mut Crate,
    function: &mut Function,
    path: &Path,
) -> Option<Value> {
    super::value::value(tokens, function)
}
