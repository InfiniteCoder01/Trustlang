use super::Token;
use super::TokenStream;
use crate::CompilationError;
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

    pub fn next_token(&mut self) -> Option<Token> {
        self.fill_token();
        self.peek.take()
    }

    pub fn peek_token(&mut self) -> Option<&Token> {
        self.fill_token();
        self.peek.as_ref()
    }

    pub fn next_token_if(&mut self, pred: impl FnOnce(&Token) -> bool) -> Option<Token> {
        self.fill_token();
        if self.peek.as_ref().is_some_and(pred) {
            self.peek.take()
        } else {
            None
        }
    }

    pub fn match_keyword(&mut self, expectation: super::Keyword) -> bool {
        self.fill_token();
        self.next_token_if(|token| {
            if let Token::Ident(_, Some(keyword)) = token {
                keyword == &expectation
            } else {
                false
            }
        })
        .is_some()
    }

    fn fill_token(&mut self) {
        if self.peek.is_none() {
            self.peek = self.token_stream.next_token();
        }
    }
}

// * ------------------------------------ Errors ------------------------------------ * //
impl<R: Read> TokenBuffer<R> {
    pub fn error(&mut self, message: impl Into<String>) {
        self.token_stream.error(message)
    }

    pub fn errors(&self) -> &[CompilationError] {
        self.token_stream.errors()
    }

    pub fn take_errors(self) -> Vec<CompilationError> {
        self.token_stream.take_errors()
    }
}
