use std::{fs, io, path::Path};

use crate::{Cursor, ParseError, ParseToken, Revert, Token};

#[derive(Debug, Default, Clone)]
pub struct Stream {
    pub prev: Token,
    pub curr: Token,
    pub next: Token,
    pub cursor: Cursor,
}

impl Stream {
    pub fn next(&mut self) -> Option<Token> {
        return self.next_of_type::<Token>();
    }

    pub fn next_of_type<T: ParseToken>(&mut self) -> Option<Token> {
        self.cursor.start = self.cursor.end;
        self.cursor.next();
        let token = T::parse(&mut self.cursor)?;
        self.prev = self.curr.clone();
        self.curr = self.next.clone();
        self.next = token.clone();
        log::debug!(target: "cmark:stream", r#""{}" => "{}" => "{}""#, self.prev, self.curr, self.next);
        return Some(token);
    }

    pub fn next_if(&mut self, value: &str) -> bool {
        if self.curr != value {
            return false;
        }

        self.next();
        return true;
    }

    pub fn next_or_err(&mut self, value: &str) -> Option<ParseError> {
        if self.next_if(value) {
            return None;
        }

        return Some(ParseError::from_str(
            self.prev.start(),
            self.prev.end(),
            format!(r#"expected "{}", found "{}""#, value, self.curr).as_str(),
        ));
    }

    pub fn next_while(&mut self, value: &str) -> u32 {
        let mut i = 0;

        while self.next_if(value) {
            i += 1;
        }

        return i;
    }

    pub fn next_until(&mut self, value: &str) -> u32 {
        let mut i = 0;

        while !self.next_if(value) {
            i += 1;
        }

        return i;
    }

    pub fn next_n(&mut self, value: &str, n: u32) -> bool {
        let mut copy = self.clone();

        for _ in 0..n {
            if !self.next_if(value) {
                self.revert(&mut copy);
                return false;
            }
        }

        return true;
    }

    pub fn is_eof(&self) -> bool {
        return self.curr.is_eof();
    }

    pub fn is_sof(&self) -> bool {
        return self.curr.is_sof();
    }

    pub fn eof(&self) -> ParseError {
        return ParseError::eof(self.cursor.start, self.cursor.end);
    }

    pub fn ignore(&self) -> ParseError {
        return ParseError::ignore(self.cursor.start, self.cursor.end);
    }
}

impl From<Vec<u8>> for Stream {
    fn from(value: Vec<u8>) -> Self {
        return Self::from(Cursor::from(value));
    }
}

impl From<&str> for Stream {
    fn from(value: &str) -> Self {
        return Self::from(Cursor::from(value.as_bytes().to_vec()));
    }
}

impl TryFrom<&Path> for Stream {
    type Error = io::Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        return Ok(Self::from(Cursor::from(fs::read(value)?)));
    }
}

impl From<Cursor> for Stream {
    fn from(cursor: Cursor) -> Self {
        let mut value = Self {
            prev: Token::default(),
            curr: Token::default(),
            next: Token::default(),
            cursor,
        };

        while value.is_sof() {
            value.next();
        }

        return value;
    }
}

impl Revert for Stream {
    fn revert(&mut self, to: &mut Self) {
        self.prev = to.prev.clone();
        self.curr = to.curr.clone();
        self.next = to.next.clone();
        self.cursor.revert(&mut to.cursor);
    }
}
