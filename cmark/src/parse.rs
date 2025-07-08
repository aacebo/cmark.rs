use crate::{cursor::Cursor, parse_error::ParseError, render::Render};

pub trait Parse<T: Render> {
    fn parse(&self, cursor: &mut Cursor) -> Result<T, ParseError>;
}
