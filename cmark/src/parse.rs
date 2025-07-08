use crate::{cursor::Cursor, parse_error::ParseError, render::Render};

pub trait Parse {
    fn parse(&self, cursor: &mut Cursor) -> Result<Box<dyn Render>, ParseError>;
}
