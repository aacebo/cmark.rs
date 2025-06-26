use std::error::Error;

use super::{position::Position, token::Token, token_pointer::TokenPointer};

#[derive(Debug, Clone)]
pub struct Pointer {
    pub src: Vec<u8>,
    pub start: Position,
    pub end: Position,
    pub tokens: TokenPointer,
}

impl Pointer {
    pub fn new(src: Vec<u8>) -> Self {
        return Self {
            src,
            start: Position::default(),
            end: Position::default(),
            tokens: TokenPointer::default(),
        };
    }

    pub fn is_sof(&self) -> bool {
        return self.start.index == 0;
    }

    pub fn is_eof(&self) -> bool {
        return self.end.index >= self.src.len();
    }

    pub fn curr(&self) -> u8 {
        if self.start.index >= self.src.len() {
            return 0;
        }

        return self.src[self.start.index];
    }

    pub fn peek(&self) -> u8 {
        if self.is_eof() {
            return 0;
        }

        return self.src[self.end.index];
    }

    pub fn next(&mut self) -> u8 {
        self.end.index += 1;
        self.end.col += 1;

        if self.peek() == b'\n' {
            self.end.ln += 1;
            self.end.col = 0;
        }

        return self.peek();
    }

    pub fn to_bytes(&self) -> &[u8] {
        return &self.src[self.start.index..self.end.index];
    }

    pub fn to_error(&self, message: &str) -> Box<dyn Error> {
        return Box::new(crate::error::Error::from_str(self.start, self.end, message));
    }

    pub fn create(&mut self, kind: u8) -> Token {
        let token = Token::new(kind, self.start, self.end, Vec::from(self.to_bytes()));
        self.tokens.next(token.clone());
        return token;
    }
}
