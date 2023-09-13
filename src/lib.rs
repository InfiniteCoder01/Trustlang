pub use lexer::Cursor;
pub use lexer::TokenStream;
pub use parser::Program;
pub mod lexer;
pub mod parser;

pub fn parse<T: std::io::Read>(source: T, sourcepath: Option<&str>) -> Vec<lexer::SpannedToken> {
    let mut tokens = Vec::new();
    let mut lexer = TokenStream::new(source, sourcepath);
    while let Some(token) = lexer.next_token().map_err(|err| panic!("{}", err)).unwrap() {
        tokens.push(token);
    }
    tokens
}

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
