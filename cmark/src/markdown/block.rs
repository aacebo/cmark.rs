use crate::{ParseError, html};

#[derive(Debug, Clone, Copy)]
pub enum Block {}

impl html::Parse for Block {
    fn parse(_stream: &mut html::Stream) -> Result<html::Node, ParseError> {
        unimplemented!()
    }
}
