use crate::lexer::*;
use crate::parser::expression::Expression;
use crate::parser::{Crate, Path};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Function {
    pub path: Path,
    pub namespan: Span,
    pub body: Vec<Expression>,
}

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
            body: Vec::new(),
        };
        match crate::parser::expression::block::parse(tokens, crate_, &mut function, &path) {
            Some(tail_return) => function.body.push(Expression::Return(tail_return)),
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
            writeln!(f, "    {expression}")?;
        }
        Ok(())
    }
}
