use crate::{ParseError, html, token};

#[derive(Debug, Clone, Copy)]
pub struct Bold;

impl html::Parse for Bold {
    fn parse(stream: &mut html::Stream) -> Result<html::Node, ParseError> {
        let mut el = html::Element::new("strong");

        if !stream.scan_n::<token![*]>(2) {
            return Err(stream.err(r#"expected "**""#));
        }

        while !stream.scan_n::<token![*]>(2) {
            let node = stream.parse::<super::Inline>()?;
            el.push(node);
        }

        return Ok(html::Node::from(el));
    }
}
