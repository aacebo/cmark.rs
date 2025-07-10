use crate::{ParseError, html, tokens::Iter};

#[derive(Clone)]
pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        return Self;
    }

    pub fn parse(&self, _src: Vec<u8>) -> Result<html::Node, ParseError> {
        unimplemented!()
    }

    pub fn parse_file(&self, _path: &'_ str) -> Result<html::Node, ParseError> {
        unimplemented!()
    }

    pub fn parse_dir(&self, _path: &'_ str) -> Result<html::Node, ParseError> {
        unimplemented!()
    }

    pub(crate) fn _parse(&self, _iter: &mut Iter) -> Result<html::Node, ParseError> {
        unimplemented!()
    }
}
