use crate::{Cursor, Parse, ParseError, html::Node};

pub trait Extension {
    fn name(&self) -> &'static str;
    fn parse(&self, parser: Box<dyn Parse>, cursor: &mut Cursor) -> Result<Node, ParseError>;
}
