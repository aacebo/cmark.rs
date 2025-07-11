use std::fmt;

use crate::{
    ParseError, Render,
    html::{self, Node},
};

#[derive(Debug, Clone)]
pub struct Stream<'a>(Vec<html::Node<'a>>);

impl<'a> Stream<'a> {
    pub fn new() -> Self {
        return Self { 0: vec![] };
    }

    pub fn is_empty(&self) -> bool {
        return self.0.is_empty();
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn get(&self, index: usize) -> Option<&Node<'a>> {
        return self.0.get(index);
    }

    pub fn iter(&self) -> Iter<'_> {
        return Iter::new(self);
    }

    pub fn push(&mut self, node: Node<'a>) {
        self.0.push(node);
    }

    pub fn append(&mut self, stream: &'a Stream) {
        for node in stream.iter() {
            self.0.push(node.clone());
        }
    }

    pub fn parse<T: Parse>(&mut self) -> Result<html::Node, ParseError> {
        let node = T::parse(self)?;
        self.push(node.clone());
        return Ok(node);
    }
}

impl<'a> FromIterator<Node<'a>> for Stream<'a> {
    fn from_iter<I: IntoIterator<Item = Node<'a>>>(iter: I) -> Self {
        return Self {
            0: iter.into_iter().collect::<Vec<Node>>(),
        };
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

#[derive(Debug, Clone)]
pub struct Iter<'t> {
    stream: &'t Stream<'t>,
    index: usize,
}

impl<'t> Iter<'t> {
    fn new(stream: &'t Stream<'t>) -> Self {
        return Self { stream, index: 0 };
    }

    pub fn peek(&self) -> Option<&Node<'t>> {
        return self.stream.0.get(self.index);
    }
}

impl<'t> Iterator for Iter<'t> {
    type Item = Node<'t>;

    fn next(&mut self) -> Option<Node<'t>> {
        return self.stream.0.get(self.index).map(|tree| {
            self.index += 1;
            tree.clone()
        });
    }
}

pub trait Parse {
    fn parse<'a>(stream: &mut Stream) -> Result<html::Node<'a>, ParseError>;
}
