use std::fmt;

use crate::{
    ParseError, ParseOptions, Render, RenderOptions, Stream,
    html::{self, ToHtml},
};

#[derive(Debug, Clone)]
pub struct BreakLine;

impl BreakLine {
    pub fn new() -> Self {
        return Self;
    }

    pub fn parse(stream: &mut Stream, _options: &ParseOptions) -> Result<Self, ParseError> {
        if !(stream.next_n(" ", 2) && stream.next_if("\n")) {
            return Err(stream.ignore());
        }

        log::debug!(target: "cmark:md:break_line", "parse");
        return Ok(Self::new());
    }
}

impl Render for BreakLine {
    fn render_into(
        &self,
        writer: &mut dyn fmt::Write,
        options: &RenderOptions,
    ) -> Result<(), fmt::Error> {
        return self.to_html().render_into(writer, options);
    }
}

impl html::ToHtml for BreakLine {
    fn to_html(&self) -> html::Node {
        return html::Element::new("br").to_html();
    }
}

impl fmt::Display for BreakLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return self.render_into(f, &RenderOptions::default());
    }
}
