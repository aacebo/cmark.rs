use std::{fmt, fs, io, path::Path};

use crate::{Cursor, Iter, ParseError, tokens::*};

#[derive(Debug, Default, Clone)]
pub struct Stream {
    pub prev: Token,
    pub curr: Token,
    pub cursor: Cursor,
}

impl Stream {
    pub fn from_src(src: Vec<u8>) -> Self {
        return Self::from(Cursor::from(src));
    }

    pub fn from_file(path: &Path) -> Result<Self, io::Error> {
        return Ok(Self::from(Cursor::from(fs::read(path)?)));
    }
}

impl Iter<&str, Token> for Stream {
    fn next(&mut self) -> Option<Token> {
        self.cursor.start = self.cursor.end;
        let byte = match self.cursor.next() {
            Some(b) => b,
            None => return None,
        };

        return match byte {
            b' ' => Space::parse(&mut self.cursor),
            b'\n' => NewLine::parse(&mut self.cursor),
            b'\t' => Tab::parse(&mut self.cursor),
            b':' => Colon::parse(&mut self.cursor),
            _ => None,
        };
    }

    fn next_if(&mut self, value: &'_ str) -> Option<Token> {
        if self.curr != value {
            return None;
        }

        self.next();
        return Some(self.prev.clone());
    }

    fn next_or_err(&mut self, value: &'_ str) -> Result<Token, ParseError> {
        return match self.next_if(value) {
            Some(token) => Ok(token),
            None => Err(ParseError::from_str(
                self.prev.start(),
                self.prev.end(),
                format!(r#"expected "{}", found "{}""#, value, self.curr).as_str(),
            )),
        };
    }

    fn next_while(&mut self, value: &'_ str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        loop {
            match self.next_if(value) {
                Some(token) => tokens.push(token),
                None => return tokens,
            };
        }
    }

    fn next_until(&mut self, value: &'_ str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while self.curr != value {
            match self.next() {
                Some(token) => tokens.push(token),
                None => return tokens,
            };
        }

        return tokens;
    }

    fn next_n(&mut self, value: &'_ str, n: i32) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        for _ in 0..n {
            match self.next_if(value) {
                Some(token) => tokens.push(token),
                None => return tokens,
            };
        }

        return tokens;
    }
}

impl From<Cursor> for Stream {
    fn from(cursor: Cursor) -> Self {
        let mut value = Self {
            prev: Token::default(),
            curr: Token::default(),
            cursor,
        };

        value.next();
        return value;
    }
}

impl fmt::Display for Stream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "prev => {}\ncurr => {}\n", self.prev, self.curr);
    }
}
