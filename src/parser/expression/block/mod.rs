use super::Value;
use crate::lexer::*;
use crate::parser::{item::function::Function, Crate, Path};

pub fn parse(
    tokens: &mut TokenBuffer,
    crate_: &mut Crate,
    function: &mut Function,
    path: &Path,
) -> Option<Value> {
    // let span = tokens.cursor();
    if tokens.match_operator(Operator::LBrace) {
        while !tokens.match_operator(Operator::RBrace) {
            // * Items
            if crate::parser::item::parse(tokens, crate_, path) {
                continue;
            }

            // * Expressions
            if let Some(value) = blocked(tokens, crate_, function, path) {
                if tokens.match_operator(Operator::RBrace) {
                    // Tail expression
                    return Some(value);
                }
            } else if let Some(value) = super::parse(tokens, crate_, function, path) {
                if tokens.match_operator(Operator::RBrace) {
                    // Tail expression
                    return Some(value);
                } else if !tokens.match_operator(Operator::Semicolon) {
                    tokens.emit_expected("';' or '}}'");
                }
            } else {
                tokens.emit_expected("an expression or a statement");
            }
        }
        Some(Value::Unit)
    } else {
        None
    }
}

/// if, for, while, block, closure, asyncs
pub fn blocked(
    tokens: &mut TokenBuffer,
    crate_: &mut Crate,
    function: &mut Function,
    path: &Path,
) -> Option<Value> {
    parse(tokens, crate_, function, path)
}
