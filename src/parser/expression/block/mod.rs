use super::Expression;
use crate::lexer::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Block {
    span: Span,
    statements: Vec<Statement>,
    tail_return: Expression,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Statement {
    Item(crate::parser::item::Item),
    Expression(Expression),
}

pub fn parse(tokens: &mut TokenBuffer) -> Option<Block> {
    let span = tokens.cursor();
    if tokens.match_operator(Operator::LBrace) {
        let mut statements = Vec::new();
        while !tokens.match_operator(Operator::RBrace) {
            if let Some(item) = crate::parser::item::parse(tokens) {
                // Item Statement
                statements.push(Statement::Item(item));
            } else if let Some(expression) = super::parse(tokens) {
                if tokens.match_operator(Operator::RBrace) {
                    // Tail Return
                    return Some(Block {
                        span: span..tokens.cursor(),
                        statements,
                        tail_return: expression,
                    });
                } else if tokens.match_operator(Operator::Semicolon) || expression.is_block() {
                    // Expression Statement
                    statements.push(Statement::Expression(expression));
                } else {
                    // Unterminated expression statement or tail return
                    let got = tokens.got_token();
                    tokens.error(format!("expected ';' or '}}', got {got}"));
                    return None;
                }
            } else {
                // Expected expression or statement
                let got = tokens.got_token();
                tokens.error(format!("expected an expression or statement, got {got}"));
            }
        }
        Some(Block {
            span: span..tokens.cursor(),
            statements,
            tail_return: Expression::Tuple(Vec::new()),
        })
    } else {
        None
    }
}
