use std::fmt;

use crate::{Cursor, ParseError, tokens::Token};

#[derive(Debug, Clone)]
pub struct Scanner {
    pub prev: Token,
    pub curr: Token,
    pub cursor: Cursor,
}

impl Scanner {
    pub fn next_if(&mut self, value: &'_ str) -> Option<Token> {
        if self.curr != value {
            return None;
        }

        self.next();
        return Some(self.prev.clone());
    }

    pub fn next_or_err(&mut self, value: &'_ str) -> Result<Token, ParseError> {
        return match self.next_if(value) {
            Some(token) => Ok(token),
            None => Err(ParseError::from_str(
                self.prev.start(),
                self.prev.end(),
                format!(r#"expected "{}", found "{}""#, value, self.curr).as_str(),
            )),
        };
    }

    pub fn next_while(&mut self, key: &'_ str) -> i32 {
        let mut i = 0;

        while self.next_if(key).is_some() {
            i = i + 1;
        }

        return i;
    }

    pub fn next_until(&mut self, key: &'_ str) -> i32 {
        let mut i = 0;

        while self.curr != key {
            self.next();
            i = i + 1;
        }

        return i;
    }

    pub fn next_n(&mut self, key: &'_ str, n: i32) -> i32 {
        let mut i = 0;

        while self.next_if(key).is_some() && i < n {
            i = i + 1;
        }

        return i;
    }
}

impl From<Cursor> for Scanner {
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

impl fmt::Display for Scanner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "prev => {}\ncurr => {}\n", self.prev, self.curr);
    }
}

impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.cursor.start = self.cursor.end;
        self.cursor.next();
        return None;
    }
}
