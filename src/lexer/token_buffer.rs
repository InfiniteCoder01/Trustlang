use super::Result;
use super::SpannedToken;
use crate::SpannedError;
use crate::TokenStream;

pub struct TokenBuffer<R: std::io::Read> {
    peek: Option<SpannedToken>,
    token_stream: TokenStream<R>,
}

impl<R: std::io::Read> TokenBuffer<R> {
    pub fn new(token_stream: TokenStream<R>) -> Self {
        Self {
            peek: None,
            token_stream,
        }
    }

    pub fn next_token(&mut self) -> Result<Option<SpannedToken>> {
        self.fill_token()?;
        Ok(self.peek.take())
    }

    pub fn peek_token(&mut self) -> Result<Option<SpannedToken>> {
        self.fill_token()?;
        Ok(self.peek.clone())
    }

    fn fill_token(&mut self) -> Result<()> {
        if self.peek.is_none() {
            self.peek = self.token_stream.next_token()?;
        }
        Ok(())
    }
}

// * ------------------------------------ Errors ------------------------------------ * //
impl<R: std::io::Read> TokenBuffer<R> {
    pub fn span<T, E>(
        &mut self,
        result: impl Into<std::result::Result<T, SpannedError<E>>>,
    ) -> std::result::Result<T, SpannedError<E>> {
        self.token_stream.span(result)
    }

    pub fn span_e<T, E>(
        &mut self,
        error: impl Into<SpannedError<E>>,
    ) -> std::result::Result<T, SpannedError<E>> {
        self.token_stream.span_e(error)
    }
}
