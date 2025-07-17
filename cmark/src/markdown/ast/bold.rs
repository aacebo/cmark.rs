use std::fmt;

use crate::{
    ParseError, ParseOptions, Render, Stream,
    html::{self, ToHtml},
    md_token,
};

#[derive(Debug, Clone)]
pub struct Bold(Vec<super::Inline>);

impl Bold {
    pub fn new() -> Self {
        return Self(vec![]);
    }

    pub fn push(&mut self, child: super::Inline) {
        self.0.push(child);
    }

    pub fn parse(stream: &mut Stream, options: &ParseOptions) -> Result<Self, ParseError> {
        let mut value = Self::new();

        log::debug!(target: "cmark:md:bold", "start parse {} => {}", stream.tokens().prev, stream.tokens().curr);

        if !stream.scan_n::<md_token![*]>(2) {
            return Err(stream.ignore());
        }

        log::debug!(target: "cmark:md:bold", "middle parse {} => {}", stream.tokens().prev, stream.tokens().curr);

        while !stream.scan_n::<md_token![*]>(2) {
            let node = super::Inline::parse(stream, options)?;
            value.push(node);
        }

        log::debug!(target: "cmark:md:bold", "end parse {} => {}", stream.tokens().prev, stream.tokens().curr);

        return Ok(value);
    }
}

impl Render for Bold {
    fn render_into(&self, writer: &mut dyn fmt::Write) -> Result<(), fmt::Error> {
        return self.to_html().render_into(writer);
    }
}

impl html::ToHtml for Bold {
    fn to_html(&self) -> html::Node {
        let mut el = html::Element::new("strong");

        for child in self.0.iter() {
            el.push(child.to_html());
        }

        return el.to_html();
    }
}

impl fmt::Display for Bold {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return self.render_into(f);
    }
}
