use crate::{ParseError, html, token};

#[derive(Debug, Clone, Copy)]
pub struct Italic;

impl html::Parse for Italic {
    fn parse(stream: &mut html::Stream) -> Result<html::Node, ParseError> {
        let mut el = html::Element::new("i");

        if !stream.scan::<token![*]>().is_some() {
            return Err(stream.err(r#"expected "*""#));
        }

        while !stream.scan::<token![*]>().is_some() {
            let node = stream.parse::<super::Inline>()?;
            el.push(node);
        }

        return Ok(html::Node::from(el));
    }
}
