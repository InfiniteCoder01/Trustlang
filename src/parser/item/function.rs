use crate::lexer::*;
use crate::parser::{Crate, Path};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Function {
    pub path: Path,
    pub namespan: Span,
    // pub body: crate::parser::expression::Block,
}

pub fn expect(tokens: &mut TokenBuffer, crate_: &mut Crate, path: &Path) {
    if let Some((name, namespan)) = tokens.next_indentifier() {
        let path = path.item(name);
        if !tokens.match_operator(Operator::LParen) {
            tokens.emit_expected("'('");
            return;
        }

        // TODO: Args
        if !tokens.match_operator(Operator::RParen) {
            let diagnostic = tokens
                .expected("')'")
                .with_notes(vec![String::from("arguments are not supported yet")]);
            tokens.codebase().emit(diagnostic);
            return;
        }

        crate_.functions.push(Function { path, namespan });
        // if let Some(body) = crate::parser::expression::block::parse(tokens) {
        //     Some(Function { name, span, body })
        // } else {
        //     tokens.emit_expected("function body");
        //     None
        // }
    } else {
        tokens.emit_expected("function name");
    }
}
