use crate::{Cursor, Extension, Parse, ParseError, html};

pub struct Markdown;

impl Markdown {
    pub fn new() -> Self {
        return Self;
    }
}

impl Extension for Markdown {
    fn name(&self) -> &'static str {
        return "markdown";
    }

    fn parse<Parser: Parse>(
        &self,
        _parser: Parser,
        _cursor: &mut Cursor,
    ) -> Result<html::Node, ParseError> {
        unimplemented!();
    }
}
