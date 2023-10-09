use crate::lexer::*;
use crate::parser::{Crate, Path};
use codespan_reporting::diagnostic::*;

pub fn expect(tokens: &mut TokenBuffer, crate_: &mut Crate, path: &Path) {
    if let Some((name, namespan)) = tokens.next_indentifier() {
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
            let name = path.path.last().unwrap();
            let path = name.to_owned() + ".tr";
            let path_dir = name.to_owned() + "/mod.tr";
            if let Ok(source) = std::fs::read_to_string(&path) {
                tokens.codebase().add(path, source);
            } else if let Ok(source) = std::fs::read_to_string(&path_dir) {
                tokens.codebase().add(path_dir, source);
            } else {
                let diagnostic = Diagnostic::error().with_message(format!(
                        "unresolved module, can't find module file: {name}.tr, or {name}/mod.tr"
                    )).with_labels(vec![Label::primary(tokens.file_id(), namespan)]);
                tokens.codebase().emit(diagnostic);
            }
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
