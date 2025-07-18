use std::fmt;

use crate::{
    ParseError, ParseOptions, Render, Stream,
    html::{self, ToHtml},
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

        if !stream.next_n("*", 2) {
            return Self::parse_alt(stream, options);
        }

        log::debug!(target: "cmark:md:bold", r#"parse "{}" => "{}" => "{}""#, stream.prev, stream.curr, stream.next);

        while !stream.next_n("*", 2) {
            let node = super::Inline::parse(stream, options)?;
            value.push(node);
        }

        return Ok(value);
    }

    pub fn parse_alt(stream: &mut Stream, options: &ParseOptions) -> Result<Self, ParseError> {
        if !options.alt {
            return Err(stream.ignore());
        }

        let mut value = Self::new();

        if !stream.next_n("_", 2) {
            return Err(stream.ignore());
        }

        log::debug!(target: "cmark:md:bold", r#"parse "{}" => "{}" => "{}""#, stream.prev, stream.curr, stream.next);

        while !stream.next_n("_", 2) {
            let node = super::Inline::parse(stream, options)?;
            value.push(node);
        }

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
