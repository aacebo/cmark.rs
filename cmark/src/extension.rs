use crate::{cursor::Cursor, parse::Parse, parse_error::ParseError, render::Render};

pub trait Extension {
    fn name(&self) -> &'static str;
    fn parse(&self, parser: dyn Parse, cursor: &mut Cursor) -> Result<Box<dyn Render>, ParseError>;
}
