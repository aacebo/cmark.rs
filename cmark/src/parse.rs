use crate::{ParseError, Parser, html, tokens::Iter};

pub trait Parse {
    fn parse<'a>(&self, parser: &'a Parser, iter: &mut Iter) -> Result<html::Node<'a>, ParseError>;
}
