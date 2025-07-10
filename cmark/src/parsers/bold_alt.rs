use crate::{Iter, Parse, ParseError, Parser, html, tokens};

#[derive(Debug, Clone)]
pub struct BoldAlt;

impl Parse for BoldAlt {
    fn parse<'a>(
        &self,
        parser: &'a Parser,
        iter: &mut tokens::Iter,
    ) -> Result<html::Node<'a>, ParseError> {
        let mut el = html::Element::new("strong");

        iter.next_or_err("__")?;

        while iter.next_if("__").is_none() {
            let node = parser._parse(iter)?;
            el.push(node);
        }

        return Ok(html::Node::Element(el));
    }
}
