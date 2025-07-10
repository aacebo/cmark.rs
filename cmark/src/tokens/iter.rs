use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    Cursor, ParseError,
    tokens::{Kind, Token},
};

#[derive(Debug, Clone)]
pub struct Iter {
    pub prev: Token,
    pub curr: Token,
    pub cursor: Rc<RefCell<Cursor>>,
}

impl crate::Iter<Kind, Token> for Iter {
    fn next_if(&mut self, kind: Kind) -> Option<Token> {
        if self.curr.kind == kind {
            self.next();
            return Some(self.prev.clone());
        }

        return None;
    }

    fn next_or_err(&mut self, kind: Kind, err: &'_ str) -> Result<Token, ParseError> {
        return match self.next_if(kind) {
            Some(token) => Ok(token),
            None => Err(ParseError::from_str(self.prev.start, self.prev.end, err)),
        };
    }

    fn next_while(&mut self, kind: Kind) -> i32 {
        let mut i = 0;

        while self.next_if(kind).is_some() {
            i = i + 1;
        }

        return i;
    }
}

impl crate::Iter<&'_ str, Token> for Iter {
    fn next_if(&mut self, key: &'_ str) -> Option<Token> {
        let bytes = key.as_bytes();

        if bytes.len() != self.curr.len() {
            return None;
        }

        for i in 0..key.len() {
            if bytes[i] != self.curr.value[i] {
                return None;
            }
        }

        self.next();
        return Some(self.prev.clone());
    }

    fn next_or_err(&mut self, key: &'_ str, err: &'_ str) -> Result<Token, ParseError> {
        return match self.next_if(key) {
            Some(token) => Ok(token),
            None => Err(ParseError::from_str(self.prev.start, self.prev.end, err)),
        };
    }

    fn next_while(&mut self, key: &'_ str) -> i32 {
        let mut i = 0;

        while self.next_if(key).is_some() {
            i = i + 1;
        }

        return i;
    }
}

impl From<Rc<RefCell<Cursor>>> for Iter {
    fn from(cursor: Rc<RefCell<Cursor>>) -> Self {
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
        return None;
    }
}
