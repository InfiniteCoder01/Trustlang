use super::Result;
use super::Token;
use super::TokenStream;
use crate::SpannedError;
use std::io::Read;

pub struct TokenBuffer<R: Read> {
    peek: Option<Token>,
    token_stream: TokenStream<R>,
}

impl<R: Read> TokenBuffer<R> {
    pub fn new(token_stream: TokenStream<R>) -> Self {
        Self {
            peek: None,
            token_stream,
        }
    }

    pub fn next_token(&mut self) -> Result<Option<Token>> {
        self.fill_token()?;
        Ok(self.peek.take())
    }

    pub fn peek_token(&mut self) -> Result<Option<Token>> {
        self.fill_token()?;
        Ok(self.peek.clone())
    }

    pub fn next_token_if(&mut self, pred: impl FnOnce(&Token) -> bool) -> Result<Option<Token>> {
        self.fill_token()?;
        if self.peek.as_ref().is_some_and(pred) {
            Ok(self.peek.take())
        } else {
            Ok(None)
        }
    }

    pub fn match_keyword(&mut self, expectation: super::Keyword) -> Result<bool> {
        self.fill_token()?;
        self.next_token_if(|token| {
            if let Token::Ident(_, Some(keyword)) = token {
                keyword == &expectation
            } else {
                false
            }
        })
        .map(|token| token.is_some())
    }

    fn fill_token(&mut self) -> Result<()> {
        if self.peek.is_none() {
            self.peek = self.token_stream.next_token()?;
        }
        Ok(())
    }
}

// * ------------------------------------ Errors ------------------------------------ * //
impl<R: Read> TokenBuffer<R> {
    pub fn span<T, E>(
        &mut self,
        error: impl Into<SpannedError<E>>,
    ) -> std::result::Result<T, SpannedError<E>> {
        self.token_stream.span(error)
    }
}
