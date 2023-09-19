pub mod lexer;
pub mod parser;

pub fn parse<T: std::io::Read>(
    source: T,
    sourcepath: Option<&str>,
) -> (String, Vec<CompilationError>) {
    use lexer::{TokenBuffer, TokenStream};
    let mut tokens = TokenBuffer::new(TokenStream::new(source, sourcepath));
    // let mut backend = orecc_back::backends::x64::X64Backend::default();
    while let Some(expression) = parser::expression::parse(&mut tokens) {
        dbg!(expression);
    }
    // while parser::declaration::parse(&mut tokens, &mut backend) {
    // }
    let backend = "";
    (backend.to_string(), tokens.take_errors())
}

// * ------------------------------------ Errors ------------------------------------ * //
use lexer::Cursor;

#[derive(Debug)]
pub struct CompilationError {
    pub message: String,
    pub sourcepath: Option<String>,
    pub at: Option<Cursor>,
}

impl std::error::Error for CompilationError {}

impl std::fmt::Display for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.at, &self.sourcepath) {
            (None, Some(sourcepath)) => write!(f, "Error at {}: {}", sourcepath, self.message)?,
            (Some(at), None) => write!(f, "Error at {}:{}: {}", at.line, at.column, self.message)?,
            (Some(at), Some(sourcepath)) => write!(
                f,
                "Error at {}:{}:{}: {}",
                sourcepath, at.line, at.column, self.message
            )?,
            (None, None) => write!(f, "Error: {}", self.message)?,
        }
        Ok(())
    }
}
