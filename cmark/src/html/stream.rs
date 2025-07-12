use std::{fmt, io, path::Path};

use common::errors::ToError;

use crate::{Iter, ParseError, Render, html, tokens};

#[derive(Debug, Clone)]
pub struct Stream<'a> {
    tokens: tokens::Stream,
    nodes: Vec<html::Node<'a>>,
}

impl<'a> Stream<'a> {
    pub fn from_src(src: Vec<u8>) -> Self {
        return Self::from(tokens::Stream::from_src(src));
    }

    pub fn from_file(path: &Path) -> Result<Self, io::Error> {
        return Ok(Self::from(tokens::Stream::from_file(path)?));
    }

    pub fn is_empty(&self) -> bool {
        return self.nodes.is_empty();
    }

    pub fn len(&self) -> usize {
        return self.nodes.len();
    }

    pub fn get(&self, index: usize) -> Option<&html::Node<'a>> {
        return self.nodes.get(index);
    }

    pub fn push(&mut self, node: html::Node<'a>) {
        self.nodes.push(node);
    }

    pub fn append(&mut self, nodes: Vec<html::Node<'a>>) {
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

impl<'a> Iter<&str, tokens::Token> for Stream<'a> {
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

impl<'a> Iter<tokens::Token, tokens::Token> for Stream<'a> {
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

impl<'a> Eq for Stream<'a> {}

impl<'a> Render for Stream<'a> {
    fn render_into(&self, writer: &mut dyn fmt::Write) -> Result<(), fmt::Error> {
        for node in self.nodes.iter() {
            node.render_into(writer)?;
        }

        return Ok(());
    }
}

impl<'a> PartialEq<Stream<'_>> for Stream<'a> {
    fn eq(&self, other: &Stream) -> bool {
        return self.nodes.iter().eq(other.nodes.iter());
    }
}

impl<'a> fmt::Display for Stream<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in self.nodes.iter() {
            node.render_into(f)?;
        }

        return Ok(());
    }
}

impl<'a> From<tokens::Stream> for Stream<'a> {
    fn from(tokens: tokens::Stream) -> Self {
        return Self {
            tokens,
            nodes: vec![],
        };
    }
}
