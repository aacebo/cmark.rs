use crate::html;

#[derive(Debug, Clone, Copy)]
pub struct Bold {}

impl html::Parse for Bold {
    fn parse<'a>(_stream: html::Stream) -> Result<html::Node<'a>, crate::ParseError> {
        let el = html::Element::new("strong");
        return Ok(html::Node::Element(el));
    }
}
