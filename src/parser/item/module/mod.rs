use crate::lexer::*;
use crate::parser::{Crate, Path};

pub fn expect(tokens: &mut TokenBuffer, crate_: &mut Crate, path: &Path) {
    if let Some((name, _namespan)) = tokens.next_indentifier() {
        let path = path.item(name);
        if tokens.match_operator(Operator::LBrace) {
            while !tokens.match_operator(Operator::RBrace) {
                if tokens.eof() {
                    tokens.emit_expected("'}'");
                    break;
                }

                super::expect(tokens, crate_, &path);
            }
        } else if tokens.match_operator(Operator::Semicolon) {
            // TODO: Modules in other files
            todo!("Modules in other files");
        } else {
            tokens.emit_expected("'{{' or ';'");
        }
    } else {
        tokens.emit_expected("module name");
    }
}

pub fn expect_entire(tokens: &mut TokenBuffer, crate_: &mut Crate, path: &Path) {
    while !tokens.eof() {
        super::expect(tokens, crate_, path);
    }
}
