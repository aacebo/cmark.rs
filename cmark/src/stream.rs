use std::{io, path::Path};

use common::errors::ToError;

use crate::{Cursor, Iter, ParseError, ParseToken, Revert, Token, TokenStream};

#[derive(Debug, Clone)]
pub struct Stream {
    tokens: TokenStream,
}

impl Stream {
    pub fn cursor(&self) -> &Cursor {
        return &self.tokens.cursor;
    }

    pub fn tokens(&self) -> &TokenStream {
        return &self.tokens;
    }

    pub fn next(&mut self) -> Option<Token> {
        return self.tokens.next();
    }

    pub fn scan<T: ParseToken>(&mut self) -> bool {
        let mut copy = self.clone();

        return match self.tokens.next_of_type::<T>() {
            Some(_) => true,
            None => {
                self.revert(&mut copy);
                return false;
            }
        };
    }

    pub fn scan_n<T: ParseToken>(&mut self, n: u32) -> bool {
        let mut copy = self.clone();

        for _ in 0..n {
            log::debug!(target: "cmark:stream:scan_n", "{} => {}", self.tokens.prev, self.tokens.curr);

            if let None = self.tokens.next_of_type::<T>() {
                self.revert(&mut copy);
                return false;
            };
        }

        return true;
    }

    pub fn err(&self, message: &str) -> ParseError {
        return self.tokens.cursor.to_error(message);
    }

    pub fn eof(&self) -> ParseError {
        return ParseError::eof(self.tokens.cursor.start, self.tokens.cursor.end);
    }

    pub fn ignore(&self) -> ParseError {
        return ParseError::ignore(self.tokens.cursor.start, self.tokens.cursor.end);
    }
}

impl From<Vec<u8>> for Stream {
    fn from(value: Vec<u8>) -> Self {
        return Self::from(TokenStream::from(value));
    }
}

impl From<&str> for Stream {
    fn from(value: &str) -> Self {
        return Self::from(TokenStream::from(value));
    }
}

impl TryFrom<&Path> for Stream {
    type Error = io::Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        return Ok(Self::from(TokenStream::try_from(value)?));
    }
}

impl From<TokenStream> for Stream {
    fn from(tokens: TokenStream) -> Self {
        return Self { tokens };
    }
}

impl Iter<&str, Token> for Stream {
    fn next_if(&mut self, value: &'_ str) -> Option<Token> {
        return self.tokens.next_if(value);
    }

    fn next_or_err(&mut self, value: &'_ str) -> Result<Token, ParseError> {
        return self.tokens.next_or_err(value);
    }

    fn next_while(&mut self, value: &'_ str) -> Vec<Token> {
        return self.tokens.next_while(value);
    }

    fn next_until(&mut self, value: &'_ str) -> Vec<Token> {
        return self.tokens.next_until(value);
    }

    fn next_n(&mut self, value: &'_ str, n: u32) -> Vec<Token> {
        return self.tokens.next_n(value, n);
    }
}

impl Iter<Token, Token> for Stream {
    fn next_if(&mut self, value: Token) -> Option<Token> {
        return self.tokens.next_if(value.as_str());
    }

    fn next_or_err(&mut self, value: Token) -> Result<Token, ParseError> {
        return self.tokens.next_or_err(value.as_str());
    }

    fn next_while(&mut self, value: Token) -> Vec<Token> {
        return self.tokens.next_while(value.as_str());
    }

    fn next_until(&mut self, value: Token) -> Vec<Token> {
        return self.tokens.next_until(value.as_str());
    }

    fn next_n(&mut self, value: Token, n: u32) -> Vec<Token> {
        return self.tokens.next_n(value.as_str(), n);
    }
}

impl Revert for Stream {
    fn revert(&mut self, to: &mut Self) {
        self.tokens.revert(&mut to.tokens);
    }
}
