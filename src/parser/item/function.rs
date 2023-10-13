use crate::lexer::*;
use crate::parser::expression::{Expression, Value};
use crate::parser::{Crate, Path, SpannedExpression};
use codespan_reporting::diagnostic::*;
use orecc_front::Span;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Function {
    pub path: Path,
    pub namespan: Span,
    pub variables: Vec<VariableMeta>,
    pub body: Vec<SpannedExpression>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct VariableMeta {
    pub definition_span: Span,
}

// * ------------------------------------ Parser ------------------------------------ * //
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

        let mut function = Function {
            path: path.clone(),
            namespan,
            variables: Vec::new(),
            body: Vec::new(),
        };
        match crate::parser::expression::block::parse(tokens, crate_, &mut function, &path) {
            Some(tail) => {
                if matches!(tail, Value::Never(_) | Value::Unit(_)) {
                    if let Some(SpannedExpression {
                        expression: Expression::Return(_, true),
                        expression_span,
                        ..
                    }) = function.body.last()
                    {
                        let diagnostic = Diagnostic::warning()
                            .with_message("unneeded `return` statement")
                            .with_labels(vec![Label::primary(
                                tokens.file_id(),
                                expression_span.clone(),
                            )]);
                        tokens.codebase().emit(diagnostic)
                    }
                } else {
                    let span = tail.span(&function);
                    function.body.push(SpannedExpression {
                        expression: Expression::Return(tail, true),
                        expression_span: span.clone(),
                        operator_span: span,
                    });
                }
            }
            None => {
                tokens.emit_expected("function body");
            }
        }
        crate_.functions.push(function);
    } else {
        tokens.emit_expected("function name");
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}:", self.path)?;
        for expression in &self.body {
            writeln!(f, "    {}", expression.expression)?;
        }
        Ok(())
    }
}
