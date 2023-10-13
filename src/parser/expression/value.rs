use orecc_front::Span;

use super::super::item::function::Function;
use crate::lexer::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Value {
    Never(Span),
    Unit(Span),
    Literal(Literal, Span),
    Variable(Variable),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Target {
    Variable(Variable),
}

pub type Variable = usize;

pub fn value(tokens: &mut TokenBuffer, _function: &mut Function) -> Option<Value> {
    let token = tokens.next_token()?;
    match token.token {
        Token::Literal(literal) => Some(Value::Literal(literal, token.span)),
        _ => None,
    }
}

// * ------------------------------------- Span ------------------------------------- * //
impl Value {
    pub fn span(&self, function: &Function) -> Span {
        match self {
            Value::Never(span) => span.clone(),
            Value::Unit(span) => span.clone(),
            Value::Literal(_, span) => span.clone(),
            Value::Variable(variable) => function.variables[*variable].definition_span.clone(),
        }
    }
}

// * ------------------------------------ Display ----------------------------------- * //
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Never(_) => write!(f, "!"),
            Value::Unit(_) => write!(f, "()"),
            Value::Literal(literal, _) => write!(f, "{literal}"),
            Value::Variable(variable) => write!(f, "%{variable}"),
        }
    }
}
impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Variable(variable) => write!(f, "%{variable}"),
        }
    }
}
