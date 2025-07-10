use std::fmt::Display;

use crate::{
    Cursor, ParseError,
    tokens::{Kind, Token},
};

#[derive(Debug, Clone)]
pub struct Iter {
    pub prev: Token,
    pub curr: Token,
    pub cursor: Cursor,
}

impl crate::Iter<Kind, Token> for Iter {
    fn next_if(&mut self, kind: Kind) -> Option<Token> {
        if self.curr.kind != kind {
            return None;
        }

        self.next();
        return Some(self.prev.clone());
    }

    fn next_or_err(&mut self, kind: Kind) -> Result<Token, ParseError> {
        return match self.next_if(kind) {
            Some(token) => Ok(token),
            None => Err(ParseError::from_str(
                self.prev.start,
                self.prev.end,
                format!(r#"expected "{}", found "{}""#, kind, self.curr.kind).as_str(),
            )),
        };
    }

    fn next_while(&mut self, kind: Kind) -> i32 {
        let mut i = 0;

        while self.next_if(kind).is_some() {
            i = i + 1;
        }

        return i;
    }

    fn next_until(&mut self, kind: Kind) -> i32 {
        let mut i = 0;

        while self.curr.kind != kind {
            self.next();
            i = i + 1;
        }

        return i;
    }

    fn next_n(&mut self, kind: Kind, n: i32) -> i32 {
        let mut i = 0;

        while self.next_if(kind).is_some() && i < n {
            i = i + 1;
        }

        return i;
    }
}

impl crate::Iter<&'_ str, Token> for Iter {
    fn next_if(&mut self, key: &'_ str) -> Option<Token> {
        if self.curr != key {
            return None;
        }

        self.next();
        return Some(self.prev.clone());
    }

    fn next_or_err(&mut self, key: &'_ str) -> Result<Token, ParseError> {
        return match self.next_if(key) {
            Some(token) => Ok(token),
            None => Err(ParseError::from_str(
                self.prev.start,
                self.prev.end,
                format!(r#"expected "{}", found "{}""#, key, self.curr).as_str(),
            )),
        };
    }

    fn next_while(&mut self, key: &'_ str) -> i32 {
        let mut i = 0;

        while self.next_if(key).is_some() {
            i = i + 1;
        }

        return i;
    }

    fn next_until(&mut self, key: &'_ str) -> i32 {
        let mut i = 0;

        while self.curr != key {
            self.next();
            i = i + 1;
        }

        return i;
    }

    fn next_n(&mut self, key: &'_ str, n: i32) -> i32 {
        let mut i = 0;

        while self.next_if(key).is_some() && i < n {
            i = i + 1;
        }

        return i;
    }
}

impl From<Cursor> for Iter {
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

impl Display for Iter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "prev => {}\ncurr => {}\n", self.prev, self.curr);
    }
}

impl Iterator for Iter {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.cursor.start = self.cursor.end;
        self.cursor.next();
        return None;
    }
}
