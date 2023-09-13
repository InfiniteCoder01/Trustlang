use super::Result;
use std::io::BufReader;
use utf8_chars::BufReadCharsExt;

pub struct CharacterBuffer<R: std::io::Read> {
    source: std::io::BufReader<R>,
    path: Option<String>,
    peek: Option<char>,
    cursor: Cursor,
}

impl<R: std::io::Read> CharacterBuffer<R> {
    pub fn new(source: R, sourcepath: Option<String>) -> Self {
        Self {
            source: BufReader::new(source),
            path: sourcepath,
            peek: None,
            cursor: Cursor::default(),
        }
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn path(&self) -> &Option<String> {
        &self.path
    }

    // * Chars
    pub fn peek_char(&mut self) -> Result<Option<char>> {
        self.fill_char()?;
        Ok(self.peek)
    }

    pub fn next_char(&mut self) -> Result<Option<char>> {
        self.fill_char()?;
        if let Some(char) = self.peek {
            if char == '\n' {
                self.cursor.line += 1;
                self.cursor.column = 0;
            } else {
                self.cursor.column += 1;
            }
        }
        Ok(self.peek.take())
    }

    pub fn next_char_if(&mut self, pred: impl FnOnce(char) -> bool) -> Result<Option<char>> {
        if let Some(char) = self.peek_char()? {
            if pred(char) {
                return self.next_char();
            }
        }
        Ok(None)
    }

    pub fn next_token(&mut self, pred: impl Fn(char) -> bool, prefix: String) -> Result<String> {
        let mut buffer = prefix;
        while let Some(char) = self.next_char_if(&pred)? {
            buffer.push(char);
        }
        Ok(buffer)
    }
}

impl<R: std::io::Read> CharacterBuffer<R> {
    fn fill_char(&mut self) -> Result<()> {
        if self.peek.is_none() {
            self.peek = self
                .source
                .read_char()
                .map_err(super::LexerError::from)
                .map_err(super::SpannedError::from)
                .map_err(|mut err| {
                    err.sourcepath = self.path.clone();
                    err
                })?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Cursor {
    pub line: usize,
    pub column: usize,
}
