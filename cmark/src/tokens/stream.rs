use std::{fs, io, path::Path};

use crate::{Cursor, Iter, ParseError, tokens::*};

#[derive(Debug, Default, Clone)]
pub struct Stream {
    pub prev: Token,
    pub curr: Token,
    pub cursor: Cursor,
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

impl Iter<&str, Token> for Stream {
    fn next(&mut self) -> Option<Token> {
        self.cursor.start = self.cursor.end;
        let token = Token::parse(&mut self.cursor)?;
        self.prev = self.curr.clone();
        self.curr = token.clone();
        return Some(token);
    }

    fn next_if(&mut self, value: &'_ str) -> Option<Token> {
        if self.curr != value {
            return None;
        }

        self.next()?;
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
