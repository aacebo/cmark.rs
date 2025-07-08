use crate::{parse_error::ParseError, tokens::Token};

pub trait Scan {
    fn prev(&self) -> Token;
    fn curr(&self) -> Token;
    fn next(&self) -> bool;
    fn scan_next(&self) -> Result<Token, ParseError>;
}

impl Iterator for dyn Scan {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        return match self.scan_next() {
            Ok(v) => Some(v),
            Err(_) => None,
        };
    }
}
