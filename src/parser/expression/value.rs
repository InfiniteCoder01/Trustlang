use super::super::item::function::Function;
use crate::lexer::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Value {
    Unit,
    Literal(Literal),
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
        Token::Literal(literal) => Some(Value::Literal(literal)),
        _ => None,
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Unit => write!(f, "()"),
            Value::Literal(literal) => write!(f, "{literal}"),
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
