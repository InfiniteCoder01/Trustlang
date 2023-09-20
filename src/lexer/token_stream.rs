use super::*;
use orecc_front::chacacter_buffer::CharacterBuffer;
use orecc_front::chars;
use std::io::Read;

pub struct TokenStream<R: Read> {
    source: CharacterBuffer<R>,
    token_start: Option<Cursor>,
    errors: Vec<CompilationError>,
    sourcepath: Option<String>,
}

impl<R: Read> TokenStream<R> {
    pub fn new(source: R, sourcepath: Option<&str>) -> Self {
        Self {
            source: CharacterBuffer::new(source),
            token_start: None,
            errors: Vec::new(),
            sourcepath: sourcepath.map(str::to_owned),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        // * Skip spaces
        while self.source.next_char_if(chars::is_whitespace).is_some() {}

        loop {
            // * Parse a token
            self.token_start = Some(self.source.cursor().clone());
            let char = self.source.next_char()?;

            // * Tokens
            // TODO: Raw strings
            if chars::is_ident_start(char) {
                let ident = self.source.next_token(chars::is_ident_continue, char);
                return Some(match ident.as_str() {
                    "true" => Token::Literal(Literal::Bool(true)),
                    "false" => Token::Literal(Literal::Bool(false)),

                    "as" => Token::Ident(ident, Some(Keyword::As)),
                    "fn" => Token::Ident(ident, Some(Keyword::Fn)),

                    "bool" => Token::Ident(ident, Some(Keyword::Bool)),
                    _ => Token::Ident(ident, None),
                });
            } else if char.is_ascii_digit() {
                return Some(Token::Literal(Literal::Int(
                    match self.source.peek_char() {
                        Some('x') if char == '0' => todo!("HEX literals"),
                        Some('b') if char == '0' => todo!("BIN literals"),
                        Some('0'..='9') => self
                            .source
                            .next_token(|char| char.is_ascii_digit(), char)
                            .parse()
                            .expect("Internal error: Failed to parse a number!"),
                        _ => char
                            .to_digit(10)
                            .expect("Internal error: Failed to parse a number!")
                            as u128,
                    },
                )));
                // TODO: Number suffixes
            } else if char == '\'' {
                return Some(self.parse_char_or_lifetime());
            } else if char == '\"' {
                return Some(Token::Literal(self.parse_string()));
            } else {
                {
                    #[rustfmt::skip]
                    let mut operators = [
                        "+", "-", "*", "/", "%",
                        "&", "|", "^",
                        ";",
                        "(", ")", "{", "}",
                        "&&", "||", "<<", ">>",
                    ];

                    operators.sort();
                    let mut buffer = char.to_string();
                    if operators.contains(&buffer.as_str()) {
                        while let Some(char) = self.source.peek_char() {
                            buffer.push(char);
                            if operators.contains(&buffer.as_str()) {
                                self.source.next_char();
                            } else {
                                buffer.pop();
                                break;
                            }
                        }

                        if !operators.is_empty() {
                            return Some(Token::Operator(match buffer.as_str() {
                                "+" => Operator::Plus,
                                "-" => Operator::Minus,
                                "*" => Operator::Star,
                                "/" => Operator::Slash,
                                "%" => Operator::Modulo,

                                "&" => Operator::Ampersand,
                                "|" => Operator::Bar,
                                "^" => Operator::Carrot,

                                ";" => Operator::Semicolon,

                                "(" => Operator::LParen,
                                ")" => Operator::RParen,
                                "{" => Operator::LBrace,
                                "}" => Operator::RBrace,

                                "&&" => Operator::LogicalAnd,
                                "||" => Operator::LogicalOr,
                                "<<" => Operator::ShiftLeft,
                                ">>" => Operator::ShiftRight,
                                operator => {
                                    panic!("Internal error: operator '{operator}' is not supported")
                                }
                            }));
                        }
                    }
                }
                self.error(format!("failed to parse token that starts with {char:?}"));
            }
        }
    }

    fn parse_char_or_lifetime(&mut self) -> Token {
        if let Some(char) = self
            .source
            .next_char_if(|char| char != '\r' && char != '\n')
        {
            // TODO: escape codes
            if self.source.next_char_if(|char| char == '\'').is_some() {
                // TODO: Char suffixes
                Token::Literal(Literal::Char(char))
            } else {
                self.error("non-terminated character literal, lifetimes are not supported yet");
                Token::Literal(Literal::Char(char))
            }
        } else {
            self.error("empty character literal or lifetime");
            Token::Literal(Literal::Char('\0'))
        }
    }

    fn parse_string(&mut self) -> Literal {
        let mut buffer = String::new();
        loop {
            if let Some(char) = self
                .source
                .next_char_if(|char| char != '\r' && char != '\n')
            {
                if char == '"' {
                    break;
                }
                // TODO: escape codes, ref: https://github.com/MaxXSoft/laps/blob/3e193c16c2baf9baa65ea9b7c5d81f8d891bd858/src/lexer.rs#L229
                buffer.push(char);
            } else {
                self.error("unterminated string literal");
            }
        }
        // TODO: String suffixes
        Literal::String(buffer)
    }
}

// * ------------------------------------ Errors ------------------------------------ * //
impl<R: Read> TokenStream<R> {
    pub fn error(&mut self, message: impl Into<String>) {
        self.errors.push(CompilationError {
            message: message.into(),
            sourcepath: self.sourcepath.clone(),
            at: self.token_start.clone(),
        });
    }

    pub fn errors(&self) -> &[CompilationError] {
        self.errors.as_ref()
    }

    pub fn take_errors(self) -> Vec<CompilationError> {
        self.errors
    }
}
