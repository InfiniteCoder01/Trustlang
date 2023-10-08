use super::*;
use codespan_reporting::diagnostic::*;
use orecc_front::Codebase;
use token_stream::TokenStream;

#[derive(Debug)]
pub struct TokenBuffer<'a> {
    peek: Option<SpannedToken>,
    token_stream: TokenStream<'a>,
}

impl<'a> TokenBuffer<'a> {
    pub fn new(codebase: &'a mut Codebase, source: std::rc::Rc<str>, file_id: usize) -> Self {
        Self {
            peek: None,
            token_stream: TokenStream::new(codebase, source, file_id),
        }
    }

    pub fn cursor(&self) -> usize {
        self.token_stream.cursor()
    }

    pub fn codebase(&mut self) -> &mut Codebase {
        self.token_stream.codebase()
    }

    pub fn file_id(&self) -> usize {
        self.token_stream.file_id
    }
}

impl TokenBuffer<'_> {
    pub fn next_token(&mut self) -> Option<SpannedToken> {
        self.fill_token();
        self.peek.take()
    }

    pub fn peek_token(&mut self) -> Option<&SpannedToken> {
        self.fill_token();
        self.peek.as_ref()
    }

    pub fn next_indentifier(&mut self) -> Option<(String, Span)> {
        self.fill_token();
        if let Some(SpannedToken {
            token: Token::Ident(_, _),
            ..
        }) = &self.peek
        {
            match self.peek.take() {
                Some(SpannedToken {
                    token: Token::Ident(identifier, _),
                    span,
                }) => Some((identifier, span)),
                _ => unreachable!(),
            }
        } else {
            None
        }
    }

    pub fn next_token_if(&mut self, pred: impl FnOnce(&Token) -> bool) -> Option<SpannedToken> {
        self.fill_token();
        if self.peek.as_ref().is_some_and(|token| pred(&token.token)) {
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

    pub fn match_operator(&mut self, expectation: super::Operator) -> bool {
        self.fill_token();
        self.next_token_if(|token| {
            if let Token::Operator(operator) = token {
                operator == &expectation
            } else {
                false
            }
        })
        .is_some()
    }
}

impl TokenBuffer<'_> {
    pub fn eof(&mut self) -> bool {
        self.fill_token();
        self.peek.is_none()
    }

    pub fn got_token(&mut self) -> String {
        self.fill_token();
        self.peek
            .as_ref()
            .map_or_else(|| String::from("<eof>"), |token| token.token.to_string())
    }

    pub fn label(&mut self, style: LabelStyle) -> Label<usize> {
        self.fill_token();
        Label::new(
            style,
            self.file_id(),
            self.peek.as_ref().map_or_else(
                || self.cursor()..self.cursor() + 1,
                |token| token.span.clone(),
            ),
        )
    }

    pub fn expected(&mut self, expectation: &str) -> Diagnostic<usize> {
        let got = self.got_token();
        Diagnostic::error()
            .with_message(format!("expected {expectation}, got: {got}"))
            .with_labels(vec![self.label(LabelStyle::Primary)])
    }

    pub fn emit_expected(&mut self, expectation: &str) {
        let expectation = self.expected(expectation);
        self.codebase().emit(expectation);
    }

    fn fill_token(&mut self) {
        if self.peek.is_none() {
            self.peek = self.token_stream.next_token();
        }
    }
}
