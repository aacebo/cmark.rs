use crate::{Cursor, ParseError, html::Node};

pub trait Parse {
    fn parse(&self, cursor: &mut Cursor) -> Result<Node, ParseError>;
}
