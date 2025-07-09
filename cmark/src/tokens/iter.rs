use std::fmt::Display;

use crate::tokens::{Scan, Token};

#[derive(Debug, Clone)]
pub struct Iter<Scanner: Scan> {
    pub prev: Token,
    pub curr: Token,

    scanner: Scanner,
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
