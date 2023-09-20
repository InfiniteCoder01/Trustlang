use super::Expression;
use crate::lexer::*;
use std::io::Read;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Block {
    statements: Vec<Statement>,
    tail_return: Expression,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Statement {
    Item(crate::parser::item::Item),
    Expression(Expression),
}

pub fn parse<R: Read>(tokens: &mut TokenBuffer<R>) -> Option<Block> {
    if tokens.match_operator(Operator::LBrace) {
        let mut statements = Vec::new();
        while !tokens.match_operator(Operator::RBrace) {
            if let Some(item) = crate::parser::item::parse(tokens) {
                // Item Statement
                statements.push(Statement::Item(item));
            } else if let Some(expression) = super::parse(tokens) {
                if tokens.match_operator(Operator::Semicolon) {
                    // Expression Statement
                    statements.push(Statement::Expression(expression));
                } else if tokens.match_operator(Operator::RBrace) {
                    // Tail Return
                    return Some(Block {
                        statements,
                        tail_return: expression,
                    });
                } else {
                    // Unterminated expression statement or tail return
                    let got = tokens.got_token();
                    tokens.error(format!("expected ';' or '}}', got {}", got));
                    return None;
                }
            } else {
                // Expected expression or statement
                let got = tokens.got_token();
                tokens.error(format!("expected an expression or statement, got {}", got));
            }
        }
        Some(Block {
            statements,
            tail_return: Expression::Tuple(Vec::new()),
        })
    } else {
        None
    }
}
