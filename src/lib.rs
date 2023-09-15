pub mod lexer;
pub mod parser;

pub fn parse<T: std::io::Read>(
    source: T,
    sourcepath: Option<&str>,
) -> Result<Vec<parser::Expression>, SpannedError<parser::ParsingError>> {
    use lexer::{TokenBuffer, TokenStream};
    let mut tokens = TokenBuffer::new(TokenStream::new(source, sourcepath));
    let mut expressions = Vec::new();
    while let Some(expression) = parser::Expression::parse(&mut tokens)? {
        expressions.push(expression);
    }
    Ok(expressions)
}

// * ------------------------------------ Errors ------------------------------------ * //
use lexer::Cursor;
use thiserror::Error;

#[derive(Error, Debug)]
pub struct SpannedError<T> {
    pub error: T,
    pub sourcepath: Option<String>,
    pub at: Option<Cursor>,
}

impl<T> From<T> for SpannedError<T> {
    fn from(value: T) -> Self {
        Self {
            error: value,
            sourcepath: None,
            at: None,
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for SpannedError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(at) = &self.at {
            write!(f, "Error at {}:{}: {}", at.line, at.column, self.error)?;
        } else {
            write!(f, "Error: {}", self.error)?;
        }
        Ok(())
    }
}
