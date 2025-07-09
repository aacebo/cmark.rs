use std::fmt::Display;

use crate::{ParseError, tokens::Token};

pub trait ToStream {
    fn stream_into(&self, stream: &mut Stream) -> Result<(), ParseError>;
    fn to_stream(&self) -> Result<Stream, ParseError> {
        let mut stream = Stream::new();
        self.stream_into(&mut stream)?;
        return Ok(stream);
    }
}

#[derive(Debug, Clone, Default)]
pub struct Stream(Vec<Token>);

impl Stream {
    pub fn new() -> Self {
        return Self { 0: vec![] };
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn iter(&self) -> core::slice::Iter<Token> {
        return self.0.iter();
    }

    pub fn push(&mut self, value: Token) {
        self.0.push(value);
    }

    pub fn pop(&mut self) -> Option<Token> {
        return self.0.pop();
    }

    pub fn append(&mut self, other: &mut Vec<Token>) {
        self.0.append(other);
    }
}

impl From<Vec<Token>> for Stream {
    fn from(value: Vec<Token>) -> Self {
        return Self { 0: value };
    }
}

impl Into<Vec<Token>> for Stream {
    fn into(self) -> Vec<Token> {
        return self.0;
    }
}

impl Display for Stream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut value = String::new();

        for token in self.0.iter() {
            value = value + token.to_string().as_str();
        }

        return write!(f, "{}", value);
    }
}
