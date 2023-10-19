
use codespan::Span;

use self::expression::Expression;

pub mod expression;
pub mod item;
// pub mod types;

// * ------------------------------------- Crate ------------------------------------ * //
#[derive(Clone, Debug, Default)]
pub struct Crate {
    pub functions: Vec<item::function::Function>,
}

impl Crate {
    pub fn new() -> Self {
        Self::default()
    }
}

// * ------------------------------------- Path ------------------------------------- * //
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Path {
    pub path: Vec<String>,
}

impl Path {
    pub fn new(path: &[String]) -> Self {
        Self {
            path: path.to_vec(),
        }
    }

    pub fn item(&self, item: String) -> Self {
        let mut new_path = self.clone();
        new_path.path.push(item);
        new_path
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SpannedExpression {
    pub expression: Expression,
    pub expression_span: Span,
    pub operator_span: Span,
}

// * ------------------------------------ Display ----------------------------------- * //
impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, segment) in self.path.iter().enumerate() {
            if index == 0 {
                write!(f, "{segment}")?;
            } else {
                write!(f, "::{segment}")?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for function in &self.functions {
            writeln!(f, "{}", function)?;
        }
        Ok(())
    }
}
