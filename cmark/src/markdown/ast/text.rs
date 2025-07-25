use std::fmt;

use crate::{
    ParseError, ParseOptions, Render, RenderOptions, Stream,
    html::{self, ToHtml},
};

#[derive(Debug, Clone)]
pub struct Text(html::Raw);

impl Text {
    pub fn new() -> Self {
        return Self(html::Raw::new());
    }

    pub fn parse(stream: &mut Stream, _options: &ParseOptions) -> Result<Self, ParseError> {
        if stream.is_eof() {
            return Err(stream.ignore());
        }

        log::debug!(target: "cmark:md:text", "parse");
        let mut value = Self::new();
        let token = stream.curr.clone();
        value.0.push(token.as_str());
        stream.next();

        return Ok(value);
    }
}

impl Render for Text {
    fn render_into(
        &self,
        writer: &mut dyn fmt::Write,
        options: &RenderOptions,
    ) -> Result<(), fmt::Error> {
        return self.to_html().render_into(writer, options);
    }
}

impl html::ToHtml for Text {
    fn to_html(&self) -> html::Node {
        return self.0.clone().to_html();
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return self.render_into(f, &RenderOptions::default());
    }
}
