use crate::lexer::*;
use orecc_back::Backend;
use std::io::Read;

pub fn parse<R: Read, B: Backend>(tokens: &mut TokenBuffer<R>, backend: &mut B) -> bool {
    if tokens.match_keyword(Keyword::Fn) {
        if let Some(name) = tokens.next_indentifier() {
            if !tokens.match_operator(Operator::LParen) {
                let got = tokens.got_token();
                tokens.error(format!("expected '(', got: {}", got));
                return true;
            }
            if !tokens.match_operator(Operator::RParen) {
                let got = tokens.got_token();
                tokens.error(format!(
                    "argument list is not yet supported, for now: expected ')', got: {}",
                    got
                ));
            }
            backend.define_function(name);
            crate::parser::expression::block::parse()
        } else {
            let got = tokens.got_token();
            tokens.error(format!("expected function name, got {}", got));
        }
        true
    } else {
        false
    }
}
