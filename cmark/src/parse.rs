use crate::{ParseError, html, tokens::Iter};

pub trait Parse {
    fn parse(&self, iter: &mut Iter) -> Result<html::Node, ParseError>;
}
