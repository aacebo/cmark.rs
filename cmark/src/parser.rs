use crate::{ParseError, ParseOptions, Stream, html};

pub type Result = std::result::Result<html::Node, ParseError>;

pub trait Parser {
    fn parse(stream: &mut Stream, options: &ParseOptions) -> Result;
    fn parse_block(stream: &mut Stream, options: &ParseOptions) -> Result;
}
