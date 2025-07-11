use std::{fmt, io, path::Path};

use crate::{ParseError, Render, html, tokens};

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

    pub fn iter(&self) -> Iter<'_> {
        return Iter::new(self);
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
        let node = T::parse(self.clone())?;
        self.push(node.clone());
        return Ok(node);
    }

    pub fn next(&mut self) -> Option<tokens::Token> {
        return self.tokens.next();
    }

    pub fn next_if(&mut self, value: &'_ str) -> Option<tokens::Token> {
        return self.tokens.next_if(value);
    }

    pub fn next_or_err(&mut self, value: &'_ str) -> Result<tokens::Token, ParseError> {
        return self.tokens.next_or_err(value);
    }

    pub fn next_while(&mut self, value: &'_ str) -> Vec<tokens::Token> {
        return self.tokens.next_while(value);
    }

    pub fn next_until(&mut self, value: &'_ str) -> Vec<tokens::Token> {
        return self.tokens.next_until(value);
    }

    pub fn next_n(&mut self, value: &'_ str, n: i32) -> Vec<tokens::Token> {
        return self.tokens.next_n(value, n);
    }
}

impl<'a> Eq for Stream<'a> {}

impl<'a> Render for Stream<'a> {
    fn render_into(&self, writer: &mut dyn fmt::Write) -> Result<(), fmt::Error> {
        for node in self.iter() {
            node.render_into(writer)?;
        }

        return Ok(());
    }
}

impl<'a> PartialEq<Stream<'_>> for Stream<'a> {
    fn eq(&self, other: &Stream) -> bool {
        return self.iter().eq(other.iter());
    }
}

impl<'a> fmt::Display for Stream<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in self.iter() {
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

#[derive(Debug, Clone)]
pub struct Iter<'t> {
    stream: &'t Stream<'t>,
    index: usize,
}

impl<'t> Iter<'t> {
    fn new(stream: &'t Stream<'t>) -> Self {
        return Self { stream, index: 0 };
    }

    pub fn peek(&self) -> Option<&html::Node<'t>> {
        return self.stream.nodes.get(self.index);
    }
}

impl<'t> Iterator for Iter<'t> {
    type Item = html::Node<'t>;

    fn next(&mut self) -> Option<html::Node<'t>> {
        return self.stream.nodes.get(self.index).map(|tree| {
            self.index += 1;
            tree.clone()
        });
    }
}
