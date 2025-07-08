use crate::{Cursor, Parse, ParseError, Render};

pub trait Extension {
    fn name(&self) -> &'static str;
    fn parse(
        &self,
        parser: Box<dyn Parse>,
        cursor: &mut Cursor,
    ) -> Result<Box<dyn Render>, ParseError>;
}
