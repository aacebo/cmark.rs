use crate::{ParseError, html, markdown::*};

#[derive(Debug, Clone, Copy)]
pub struct Inline;

impl html::Parse for Inline {
    fn parse(stream: &mut html::Stream) -> Result<html::Node, ParseError> {
        if let Ok(node) = Bold::parse(stream) {
            return Ok(node);
        }

        if let Ok(node) = Italic::parse(stream) {
            return Ok(node);
        }

        if let Ok(node) = BreakLine::parse(stream) {
            return Ok(node);
        }

        return Ok(html::Node::Element(html::Element::new("")));
    }
}
