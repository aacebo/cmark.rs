use std::fmt;

use crate::{
    ParseError, ParseOptions, Render, Stream,
    html::{self, ToHtml},
    md_token,
};

#[derive(Debug, Clone)]
pub struct BreakLine;

impl BreakLine {
    pub fn new() -> Self {
        return Self;
    }

    pub fn parse(stream: &mut Stream, _options: &ParseOptions) -> Result<Self, ParseError> {
        if !(stream.scan_n::<md_token![space]>(2) && stream.scan::<md_token![newline]>()) {
            return Err(stream.ignore());
        }

        log::debug!(target: "cmark:md:break_line", "parse");
        return Ok(Self::new());
    }
}

impl Render for BreakLine {
    fn render_into(&self, writer: &mut dyn fmt::Write) -> Result<(), fmt::Error> {
        return self.to_html().render_into(writer);
    }
}

impl html::ToHtml for BreakLine {
    fn to_html(&self) -> html::Node {
        return html::Element::new("br").to_html();
    }
}

impl fmt::Display for BreakLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return self.render_into(f);
    }
}
