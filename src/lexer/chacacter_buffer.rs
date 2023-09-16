use std::io::{BufReader, Read};
use utf8_chars::BufReadCharsExt;

pub struct CharacterBuffer<R: Read> {
    source: std::io::BufReader<R>,
    path: Option<String>,
    peek: Option<char>,
    cursor: Cursor,
}

impl<R: Read> CharacterBuffer<R> {
    pub fn new(source: R, sourcepath: Option<String>) -> Self {
        Self {
            source: BufReader::new(source),
            path: sourcepath,
            peek: None,
            cursor: Cursor::new(1, 1),
        }
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn path(&self) -> &Option<String> {
        &self.path
    }

    // * Chars
    pub fn peek_char(&mut self) -> Option<char> {
        self.fill_char();
        self.peek
    }

    pub fn next_char(&mut self) -> Option<char> {
        self.fill_char();
        if let Some(char) = self.peek {
            if char == '\n' {
                self.cursor.line += 1;
                self.cursor.column = 1;
            } else {
                self.cursor.column += 1;
            }
        }
        self.peek.take()
    }

    pub fn next_char_if(&mut self, pred: impl FnOnce(char) -> bool) -> Option<char> {
        self.fill_char();
        if let Some(char) = self.peek {
            if pred(char) {
                return self.peek.take();
            }
        }
        None
    }

    pub fn next_token(&mut self, pred: impl Fn(char) -> bool, prefix: char) -> String {
        let mut buffer = prefix.to_string();
        while let Some(char) = self.next_char_if(&pred) {
            buffer.push(char);
        }
        buffer
    }
}

impl<R: Read> CharacterBuffer<R> {
    fn fill_char(&mut self) {
        if self.peek.is_none() {
            self.peek = self
                .source
                .read_char()
                .expect("reading code from source failed");
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cursor {
    pub line: usize,
    pub column: usize,
}

impl Cursor {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}
