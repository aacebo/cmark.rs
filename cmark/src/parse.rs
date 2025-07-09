use crate::{Cursor, ParseError, html};

pub trait Parse {
    fn parse(&self, cursor: &mut Cursor) -> Result<html::Node, ParseError>;
}
