use super::Item;
use crate::lexer::*;
use std::io::Read;

pub fn parse<R: Read>(tokens: &mut TokenBuffer<R>) -> Option<Item> {
    if tokens.match_keyword(Keyword::Fn) {
        if let Some(name) = tokens.next_indentifier() {
            if !tokens.match_operator(Operator::LParen) {
                let got = tokens.got_token();
                tokens.error(format!("expected '(', got: {}", got));
                return None;
            }
            if !tokens.match_operator(Operator::RParen) {
                let got = tokens.got_token();
                tokens.error(format!(
                    "argument list is not yet supported, for now: expected ')', got: {}",
                    got
                ));
            }
            if let Some(body) = crate::parser::expression::block::parse(tokens) {
                Some(Item::Function(name, body))
            } else {
                let got = tokens.got_token();
                tokens.error(format!("expected function body, got {}", got));
                None
            }
        } else {
            let got = tokens.got_token();
            tokens.error(format!("expected function name, got {}", got));
            None
        }
    } else {
        None
    }
}
