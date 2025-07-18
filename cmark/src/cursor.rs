use common::errors::ToError;

use crate::{ParseError, Position, Revert};

#[derive(Debug, Clone, Default)]
pub struct Cursor {
    pub src: Vec<u8>,
    pub start: Position,
    pub end: Position,
}

impl Cursor {
    pub fn len(&self) -> usize {
        return self.end.index - self.start.index;
    }

    pub fn is_sof(&self) -> bool {
        return self.start.index == 0 && self.end.index == 0;
    }

    pub fn is_eof(&self) -> bool {
        return self.end.index > self.src.len();
    }

    pub fn start(&self) -> u8 {
        if self.start.index >= self.src.len() {
            return 0;
        }

        return self.src[self.start.index];
    }

    pub fn end(&self) -> u8 {
        if self.end.index >= self.src.len() {
            return 0;
        }

        return self.src[self.end.index];
    }

    pub fn peek(&self) -> u8 {
        if self.end.index + 1 >= self.src.len() {
            return 0;
        }

        return self.src[self.end.index + 1];
    }

    pub fn to_bytes(&self) -> &[u8] {
        return &self.src[self.start.index..self.end.index];
    }

    pub fn to_str(&self) -> Result<&str, ParseError> {
        return match std::str::from_utf8(self.to_bytes()) {
            Ok(v) => Ok(v),
            Err(err) => Err(ParseError::from_string(
                self.start,
                self.end,
                err.to_string(),
            )),
        };
    }

    pub fn next_if(&mut self, value: &str) -> bool {
        let mut copy = self.clone();

        while self.end.index < self.src.len() && self.len() < value.len() {
            self.next();
        }

        let eq = self.to_bytes() == value.as_bytes();

        if !eq {
            self.revert(&mut copy);
        }

        return eq;
    }

    pub fn is_alpha(&self, value: u8) -> bool {
        return (value >= b'a' && value <= b'z') || (value >= b'A' && value <= b'Z');
    }

    pub fn is_num(&self, value: u8) -> bool {
        return value >= b'0' && value <= b'9';
    }

    pub fn next_while_alpha(&mut self) -> u32 {
        let mut i = 0;

        while self.is_alpha(self.end()) {
            self.next();
            i = i + 1;
        }

        return i;
    }

    pub fn next_while_num(&mut self) -> u32 {
        let mut i = 0;

        while self.is_num(self.end()) {
            self.next();
            i = i + 1;
        }

        return i;
    }
}

impl From<Vec<u8>> for Cursor {
    fn from(src: Vec<u8>) -> Self {
        return Self {
            src,
            start: Position::default(),
            end: Position::default(),
        };
    }
}

impl Iterator for Cursor {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let prev = self.end();
        self.end.index += 1;
        self.end.col += 1;

        if self.end() == b'\n' {
            self.end.ln += 1;
            self.end.col = 0;
        }

        if self.end() == 0 {
            return None;
        }

        log::debug!(target: "cmark:cursor", r#""{}" => "{}""#, prev as char, self.end() as char);
        return Some(self.end());
    }
}

impl ToError<ParseError> for Cursor {
    fn to_error(&self, message: &str) -> ParseError {
        return ParseError::from_str(self.start, self.end, message);
    }
}

impl Revert for Cursor {
    fn revert(&mut self, to: &mut Self) {
        self.start = to.start;
        self.end = to.end;
    }
}
