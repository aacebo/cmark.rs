use crate::{ParseError, html, token};

#[derive(Debug, Clone, Copy)]
pub struct Bold {}

impl html::Parse for Bold {
    fn parse<'a>(stream: &mut html::Stream) -> Result<html::Node<'a>, ParseError> {
        let el = html::Element::new("strong");

        if !stream.scan_n::<token![*]>(2) {
            return Err(stream.err(r#"expected "**""#));
        }

        while !stream.scan_n::<token![*]>(2) {}

        return Ok(html::Node::Element(el));
    }
}
