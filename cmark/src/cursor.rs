use common::errors::ToError;

use crate::{ParseError, Position};

#[derive(Debug, Clone, Default)]
pub struct Cursor {
    pub src: Vec<u8>,
    pub start: Position,
    pub end: Position,
}

impl Cursor {
    pub fn is_sof(&self) -> bool {
        return self.start.index == 0;
    }

    pub fn is_eof(&self) -> bool {
        return self.end.index >= self.src.len();
    }

    pub fn curr(&self) -> u8 {
        if self.start.index >= self.src.len() {
            return 0;
        }

        return self.src[self.start.index];
    }

    pub fn peek(&self) -> u8 {
        return self.peek_n(0);
    }

    pub fn peek_n(&self, n: usize) -> u8 {
        if self.end.index + n >= self.src.len() {
            return 0;
        }

        return self.src[self.end.index + n];
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

    pub fn next_if(&mut self, value: &'_ str) -> bool {
        for c in value.chars() {
            if c != self.curr() as char {
                return false;
            }

            self.next();
        }

        return true;
    }

    pub fn next_while_alpha(&mut self) -> u32 {
        let mut i = 0;

        while (self.peek() >= b'a' && self.peek() <= b'z')
            || (self.peek() >= b'A' && self.peek() <= b'Z')
        {
            self.next();
            i = i + 1;
        }

        return i;
    }

    pub fn next_while_num(&mut self) -> u32 {
        let mut i = 0;

        while self.peek() >= b'0' && self.peek() <= b'9' {
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
        self.end.index += 1;
        self.end.col += 1;

        if self.peek() == b'\n' {
            self.end.ln += 1;
            self.end.col = 0;
        }

        if self.peek() == 0 {
            return None;
        }

        return Some(self.peek());
    }
}

impl ToError<ParseError> for Cursor {
    fn to_error(&self, message: &str) -> ParseError {
        return ParseError::from_str(self.start, self.end, message);
    }
}
