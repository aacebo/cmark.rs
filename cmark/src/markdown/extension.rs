use crate::{Cursor, Parse, ParseError, extension::Extension, html::Node};

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

    fn parse(&self, _parser: Box<dyn Parse>, _cursor: &mut Cursor) -> Result<Node, ParseError> {
        unimplemented!();
    }
}
