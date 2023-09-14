use super::chacacter_buffer::CharacterBuffer;
use super::*;

pub struct TokenStream<R: std::io::Read> {
    source: CharacterBuffer<R>,
    token_start: Option<Cursor>,
}

impl<R: std::io::Read> TokenStream<R> {
    pub fn new(source: R, sourcepath: Option<&str>) -> Self {
        Self {
            source: CharacterBuffer::new(source, sourcepath.map(str::to_owned)),
            token_start: None,
        }
    }

    pub fn next_token(&mut self) -> Result<Option<SpannedToken>> {
        // * Skip spaces
        while self.source.next_char_if(chars::is_whitespace)?.is_some() {}

        // * Parse a token
        let start = self.source.cursor().clone();
        self.token_start = Some(start.clone());
        if let Some(char) = self.source.next_char()? {
            // * Tokens
            // TODO: Raw strings
            let token = if chars::is_ident_start(char) {
                let ident = self
                    .source
                    .next_token(chars::is_ident_continue, char.to_string())?;
                match ident.as_str() {
                    "true" => Token::Literal(Literal::Bool(true)),
                    "false" => Token::Literal(Literal::Bool(false)),
                    "fn" => Token::Ident(ident, Some(Keyword::Fn)),
                    _ => Token::Ident(ident, None),
                }
            } else if char.is_ascii_digit() {
                Token::Literal(Literal::Int(match self.source.peek_char()? {
                    Some('x') if char == '0' => todo!("HEX literals"),
                    Some('b') if char == '0' => todo!("BIN literals"),
                    Some('0'..='9') => self
                        .source
                        .next_token(|char| char.is_ascii_digit(), char.to_string())?
                        .parse()
                        .expect("Internal error: Failed to parse a number!"),
                    _ => char
                        .to_digit(10)
                        .expect("Internal error: Failed to parse a number!"),
                }))
                // TODO: Number suffixes
            } else if char == '\'' {
                let char = self.read_string(char)?;
                if char.len() != 1 {
                    return self.span_e(LexerError::InvalidCharLiteralLength(char.len()));
                }
                Token::Literal(Literal::Char(char.chars().next().unwrap()))
                // TODO: Char suffixes
            } else if char == '\"' {
                Token::Literal(Literal::String(self.read_string(char)?))
                // TODO: String suffixes
            } else {
                return self.span_e(LexerError::DetermineToken(char));
            };

            Ok(Some(SpannedToken {
                token,
                start,
                end: self.source.cursor().clone(),
            }))
        } else {
            Ok(None)
        }
    }

    fn read_string(&mut self, quote: char) -> Result<String> {
        let mut buffer = String::new();
        loop {
            if let Some(char) = self.source.next_char()? {
                if char == quote {
                    break;
                }
                // TODO: escape codes, ref: https://github.com/MaxXSoft/laps/blob/3e193c16c2baf9baa65ea9b7c5d81f8d891bd858/src/lexer.rs#L229
                buffer.push(char);
            } else {
                return self.span_e(LexerError::UnterminatedStringLiteral);
            }
        }
        Ok(buffer)
    }
}

// * ------------------------------------ Errors ------------------------------------ * //
impl<R: std::io::Read> TokenStream<R> {
    pub fn span<T, E>(
        &mut self,
        result: impl Into<std::result::Result<T, SpannedError<E>>>,
    ) -> std::result::Result<T, SpannedError<E>> {
        result.into().map_err(|mut error| {
            error.at = self.token_start.take();
            error.sourcepath = self.source.path().clone();
            error
        })
    }

    pub fn span_e<T, E>(
        &mut self,
        error: impl Into<SpannedError<E>>,
    ) -> std::result::Result<T, SpannedError<E>> {
        self.span(Err(error.into()))
    }
}
