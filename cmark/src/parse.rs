use crate::{Cursor, ParseError, Render};

pub trait Parse {
    fn parse(&self, cursor: &mut Cursor) -> Result<Box<dyn Render>, ParseError>;
}
