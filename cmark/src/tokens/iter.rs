use std::fmt::Display;

use crate::{
    ParseError,
    tokens::{Kind, Scan, Token},
};

#[derive(Debug, Clone)]
pub struct Iter<Scanner: Scan> {
    pub prev: Token,
    pub curr: Token,

    scanner: Scanner,
}

impl<Scanner: Scan> crate::Iter<Kind, Token> for Iter<Scanner> {
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

impl<Scanner: Scan> crate::Iter<&'_ str, Token> for Iter<Scanner> {
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

impl<Scanner: Scan> From<Scanner> for Iter<Scanner> {
    fn from(scanner: Scanner) -> Self {
        let mut value = Self {
            prev: Token::default(),
            curr: Token::default(),
            scanner,
        };

        value.next();
        return value;
    }
}

impl<Scanner: Scan> Iterator for Iter<Scanner> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        return match self.scanner.scan() {
            None => None,
            Some(token) => {
                self.prev = self.curr.clone();
                self.curr = token.clone();
                return Some(token);
            }
        };
    }
}

impl<Scanner: Scan> Display for Iter<Scanner> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "prev => {}\ncurr => {}\n", self.prev, self.curr);
    }
}
