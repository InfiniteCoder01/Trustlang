use super::{Crate, Path};
use crate::lexer::*;

pub mod function;
pub mod module;

pub fn parse(tokens: &mut TokenBuffer, crate_: &mut Crate, path: &Path) -> bool {
    if tokens.match_keyword(Keyword::Fn) {
        function::expect(tokens, crate_, path);
    } else if tokens.match_keyword(Keyword::Mod) {
        module::expect(tokens, crate_, path);
    } else {
        return false;
    }
    true
}

pub fn expect(tokens: &mut TokenBuffer, crate_: &mut Crate, path: &Path) {
    if !parse(tokens, crate_, path) {
        tokens.emit_expected("an item");
    }
}
