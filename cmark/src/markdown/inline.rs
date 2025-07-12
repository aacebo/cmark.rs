use crate::{ParseError, html};

#[derive(Debug, Clone, Copy)]
pub enum Inline {}

impl html::Parse for Inline {
    fn parse(_stream: &mut html::Stream) -> Result<html::Node, ParseError> {
        unimplemented!()
    }
}
