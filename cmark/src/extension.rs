use crate::{Cursor, Parse, ParseError, html};

pub trait Extension {
    fn name(&self) -> &'static str;
    fn parse<Parser: Parse>(
        &self,
        parser: Parser,
        cursor: &mut Cursor,
    ) -> Result<html::Node, ParseError>;
}
