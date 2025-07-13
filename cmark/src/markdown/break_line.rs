use crate::{ParseError, html, token};

#[derive(Debug, Clone, Copy)]
pub struct BreakLine;

impl html::Parse for BreakLine {
    fn parse(stream: &mut html::Stream) -> Result<html::Node, ParseError> {
        let el = html::Element::new("br");

        if !(stream.scan_n::<token![space]>(2) && stream.curr() == "\n") {
            return Err(stream.err("expected two spaces and a newline"));
        }

        return Ok(html::Node::from(el));
    }
}
