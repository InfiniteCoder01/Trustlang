use super::*;
use codespan_reporting::diagnostic::*;
use orecc_front::chars;
use orecc_front::Codebase;
use orecc_front::TokenReader;

#[derive(Debug)]
pub struct TokenStream<'a> {
    source: TokenReader,
    codebase: &'a mut Codebase,
    pub file_id: usize,
}

impl<'a> TokenStream<'a> {
    pub fn new(codebase: &'a mut Codebase, source: std::rc::Rc<str>, file_id: usize) -> Self {
        Self {
            source: TokenReader::new(source),
            codebase,
            file_id,
        }
    }

    pub fn cursor(&self) -> usize {
        self.source.cursor
    }

    pub fn codebase(&mut self) -> &mut Codebase {
        self.codebase
    }
}

impl TokenStream<'_> {
    pub fn next_token(&mut self) -> Option<SpannedToken> {
        // * Skip spaces
        while self.source.next_char_if(chars::is_whitespace).is_some() {}

        loop {
            // * Parse a token
            let token_start = self.source.cursor;
            if let Some(char) = self.source.next_char() {
                // * Tokens
                // TODO: Raw strings
                let token = if chars::is_ident_start(char) {
                    // Ident
                    let ident = self.source.next_token(chars::is_ident_continue, char);
                    match ident.as_str() {
                        "true" => Token::Literal(Literal::Bool(true)),
                        "false" => Token::Literal(Literal::Bool(false)),

                        "return" => Token::Ident(ident, Some(Keyword::Return)),

                        "as" => Token::Ident(ident, Some(Keyword::As)),

                        "fn" => Token::Ident(ident, Some(Keyword::Fn)),
                        "mod" => Token::Ident(ident, Some(Keyword::Mod)),

                        "bool" => Token::Ident(ident, Some(Keyword::Bool)),
                        _ => Token::Ident(ident, None),
                    }
                } else if char.is_ascii_digit() {
                    // Numbers
                    Token::Literal(Literal::Int(match self.source.peek_char() {
                        Some('x') if char == '0' => todo!("HEX literals"),
                        Some('b') if char == '0' => todo!("BIN literals"),
                        Some('o') if char == '0' => todo!("OCT literals"),
                        Some('0'..='9') => crate::bug_result!(
                            self.codebase,
                            self.source
                                .next_token(|char| char.is_ascii_digit(), char)
                                .parse()
                                .ok(),
                            "failed to parse a number",
                            vec![self.cursor_label(LabelStyle::Primary)]
                        ),
                        _ => crate::bug_result!(
                            self.codebase,
                            char.to_digit(10),
                            "failed to parse a number",
                            vec![self.cursor_label(LabelStyle::Primary)]
                        ) as u128,
                    }))
                    // TODO: Number suffixes
                } else if char == '\'' {
                    self.parse_char_or_lifetime()
                } else if char == '\"' {
                    Token::Literal(self.parse_string())
                } else {
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

                        Token::Operator(match buffer.as_str() {
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
                        })
                    } else {
                        self.codebase.emit(
                            Diagnostic::error()
                                .with_message(format!(
                                    "failed to parse token that starts with {char:?}"
                                ))
                                .with_labels(vec![self.cursor_label(LabelStyle::Primary)]),
                        );

                        continue;
                    }
                };
                return self.span(token_start, token);
            } else {
                return None;
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
                self.codebase.emit(
                    Diagnostic::error()
                        .with_message(
                            "non-terminated character literal, lifetimes are not supported yet",
                        )
                        .with_labels(vec![self.cursor_label(LabelStyle::Primary)]),
                );

                Token::Literal(Literal::Char(char))
            }
        } else {
            // self.error("empty character literal or lifetime");
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
                self.codebase.emit(
                    Diagnostic::error()
                        .with_message("unterminated string literal")
                        .with_labels(vec![self.cursor_label(LabelStyle::Primary)]),
                );
                break;
            }
        }
        // TODO: String suffixes
        Literal::String(buffer)
    }

    fn span(&self, token_start: usize, token: Token) -> Option<SpannedToken> {
        Some(SpannedToken::new(token, token_start..self.source.cursor))
    }

    fn cursor_label(&self, style: LabelStyle) -> Label<usize> {
        Label::new(
            style,
            self.file_id,
            self.source.cursor..self.source.cursor + 1,
        )
    }
}
