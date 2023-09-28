use super::expression::BuiltValue;
use crate::lexer::*;
use orecc_back::ir::*;
use std::io::Read;

pub mod function;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Item {
    Function(function::Function),
}

pub fn parse<R: Read>(tokens: &mut TokenBuffer<R>) -> Option<Item> {
    if tokens.match_keyword(Keyword::Fn) {
        Some(Item::Function(function::expect_function(tokens)?))
    } else {
        None
    }
}

impl Item {
    pub fn build(self, module: &mut Module) {
        match self {
            Item::Function(function) => {
                let mut built_function = Function::new(function.name, Type::Void);
                let return_value = function.body.build(&module, &mut built_function);
                if let Some(data) = return_value.as_data() {
                    built_function.return_(data);
                }
                module.functions.push(built_function);
            }
        }
    }
}
