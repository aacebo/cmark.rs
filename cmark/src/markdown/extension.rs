use crate::{Parse, ParseError, Render, extension::Extension};

pub struct Markdown;

impl Markdown {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Extension for Markdown {
    fn name(&self) -> &'static str {
        return "markdown";
    }

    fn parse(
        &self,
        _parser: Box<dyn Parse>,
        _cursor: &mut crate::cursor::Cursor,
    ) -> Result<Box<dyn Render>, ParseError> {
        unimplemented!();
    }
}
