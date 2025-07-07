use std::error::Error;

use common::errors::ToError;

use crate::{lex_error::LexError, position::Position, tokens::{self, Token}};

#[derive(Debug, Clone)]
pub struct Cursor {
    pub src: Vec<u8>,
    pub start: Position,
    pub end: Position,
}

impl Cursor {
    pub fn new(src: Vec<u8>) -> Self {
        return Self {
            src,
            start: Position::default(),
            end: Position::default(),
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

    pub fn to_bytes(&self) -> &[u8] {
        return &self.src[self.start.index..self.end.index];
    }

    pub fn create(&mut self, kind: tokens::Kind) -> Token {
        let token = Token::new(kind, self.start, self.end, Vec::from(self.to_bytes()));
        return token;
    }
}

impl Iterator for Cursor {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        self.end.index += 1;
        self.end.col += 1;

        if self.peek() == b'\n' {
            self.end.ln += 1;
            self.end.col = 0;
        }

        if self.peek() == 0 {
            return None;
        }

        return Some(self.peek());
    }
}

impl ToError for Cursor {
    fn to_error(&self, message: &str) -> Box<dyn Error> {
        return Box::new(LexError::from_str(self.start, self.end, message));
    }
}