use crate::{Extension, ParseError, html};

pub type Result = std::result::Result<html::Node, ParseError>;

pub struct Parser<'a> {
    extensions: Vec<&'a dyn Extension>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        return Self { extensions: vec![] };
    }

    pub fn extend(&mut self, extension: &'a dyn Extension) {
        self.extensions.push(extension);
    }
}
