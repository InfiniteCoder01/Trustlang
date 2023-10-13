use super::{Expression, Value};
use crate::lexer::*;
use crate::parser::SpannedExpression;
use crate::parser::{item::function::Function, Crate, Path};

pub fn parse(
    tokens: &mut TokenBuffer,
    crate_: &mut Crate,
    function: &mut Function,
    path: &Path,
) -> Option<Value> {
    if let Some(op_span) = tokens.match_keyword(Keyword::Return) {
        let value = super::expect(tokens, crate_, function, path)?;
        function.body.push(SpannedExpression {
            expression: Expression::Return(value, true),
            expression_span: op_span.start..tokens.cursor(),
            operator_span: op_span.clone(),
        });
        return Some(Value::Never(op_span));
    }
    super::value::value(tokens, function)
}
