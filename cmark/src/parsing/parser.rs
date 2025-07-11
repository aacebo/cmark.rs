use crate::{ParseError, html, tokens::Scanner};

#[derive(Debug, Clone)]
pub struct Parser {
    scanner: Scanner,
}

impl Parser {
    pub fn new() -> Self {
        return Self {
            scanner: Scanner::default(),
        };
    }

    pub fn parse(&mut self, src: Vec<u8>) -> Result<Option<html::Node>, ParseError> {
        self.scanner = Scanner::new(src);
        self.scanner.next();
        return Ok(None);
    }
}
