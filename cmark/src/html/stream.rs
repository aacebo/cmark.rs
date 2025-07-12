use std::{fmt, io, path::Path};

use common::errors::ToError;

use crate::{Iter, ParseError, Render, html, tokens};

#[derive(Debug, Clone)]
pub struct Stream {
    tokens: tokens::Stream,
    nodes: Vec<html::Node>,
}

impl Stream {
    pub fn is_empty(&self) -> bool {
        return self.nodes.is_empty();
    }

    pub fn len(&self) -> usize {
        return self.nodes.len();
    }

    pub fn get(&self, index: usize) -> Option<&html::Node> {
        return self.nodes.get(index);
    }

    pub fn push(&mut self, node: html::Node) {
        self.nodes.push(node);
    }

    pub fn append(&mut self, nodes: Vec<html::Node>) {
        for node in nodes.iter() {
            self.nodes.push(node.clone());
        }
    }

    pub fn parse<T: super::Parse>(&mut self) -> Result<html::Node, ParseError> {
        let node = T::parse(self)?;
        self.push(node.clone());
        return Ok(node);
    }

    pub fn scan<T: tokens::Parse>(&mut self) -> Option<tokens::Token> {
        return T::parse(&mut self.tokens.cursor);
    }

    pub fn scan_n<T: tokens::Parse>(&mut self, n: i32) -> bool {
        for _ in 0..n {
            match T::parse(&mut self.tokens.cursor) {
                Some(_) => {}
                None => return false,
            };
        }

        return true;
    }

    pub fn err(&self, message: &'_ str) -> ParseError {
        return self.tokens.cursor.to_error(message);
    }
}

impl From<Vec<u8>> for Stream {
    fn from(value: Vec<u8>) -> Self {
        return Self::from(tokens::Stream::from_src(value));
    }
}

impl TryFrom<&Path> for Stream {
    type Error = io::Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        return Ok(Self::from(tokens::Stream::from_file(value)?));
    }
}

impl From<tokens::Stream> for Stream {
    fn from(tokens: tokens::Stream) -> Self {
        return Self {
            tokens,
            nodes: vec![],
        };
    }
}

impl Iter<&str, tokens::Token> for Stream {
    fn next(&mut self) -> Option<tokens::Token> {
        return self.tokens.next();
    }

    fn next_if(&mut self, value: &'_ str) -> Option<tokens::Token> {
        return self.tokens.next_if(value);
    }

    fn next_or_err(&mut self, value: &'_ str) -> Result<tokens::Token, ParseError> {
        return self.tokens.next_or_err(value);
    }

    fn next_while(&mut self, value: &'_ str) -> Vec<tokens::Token> {
        return self.tokens.next_while(value);
    }

    fn next_until(&mut self, value: &'_ str) -> Vec<tokens::Token> {
        return self.tokens.next_until(value);
    }

    fn next_n(&mut self, value: &'_ str, n: i32) -> Vec<tokens::Token> {
        return self.tokens.next_n(value, n);
    }
}

impl Iter<tokens::Token, tokens::Token> for Stream {
    fn next(&mut self) -> Option<tokens::Token> {
        return self.tokens.next();
    }

    fn next_if(&mut self, value: tokens::Token) -> Option<tokens::Token> {
        return self.tokens.next_if(value.as_str());
    }

    fn next_or_err(&mut self, value: tokens::Token) -> Result<tokens::Token, ParseError> {
        return self.tokens.next_or_err(value.as_str());
    }

    fn next_while(&mut self, value: tokens::Token) -> Vec<tokens::Token> {
        return self.tokens.next_while(value.as_str());
    }

    fn next_until(&mut self, value: tokens::Token) -> Vec<tokens::Token> {
        return self.tokens.next_until(value.as_str());
    }

    fn next_n(&mut self, value: tokens::Token, n: i32) -> Vec<tokens::Token> {
        return self.tokens.next_n(value.as_str(), n);
    }
}

impl Eq for Stream {}

impl Render for Stream {
    fn render_into(&self, writer: &mut dyn fmt::Write) -> Result<(), fmt::Error> {
        for node in self.nodes.iter() {
            node.render_into(writer)?;
        }

        return Ok(());
    }
}

impl PartialEq<Stream> for Stream {
    fn eq(&self, other: &Stream) -> bool {
        return self.nodes.iter().eq(other.nodes.iter());
    }
}

impl fmt::Display for Stream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in self.nodes.iter() {
            node.render_into(f)?;
        }

        return Ok(());
    }
}
