use std::fmt;

use crate::{
    ParseError, ParseOptions, Render, RenderOptions, Stream,
    html::{self, ToHtml},
};

#[derive(Debug, Clone)]
pub struct BlockQuote(Vec<super::Block>);

impl BlockQuote {
    pub fn new() -> Self {
        return Self(vec![]);
    }

    pub fn push(&mut self, child: super::Block) {
        self.0.push(child);
    }

    pub fn parse(stream: &mut Stream, options: &ParseOptions) -> Result<Self, ParseError> {
        let mut value = Self::new();

        if !stream.next_if(">") {
            return Err(stream.ignore());
        }

        log::debug!(target: "cmark:md:block_quote", "parse");

        loop {
            match super::Block::parse(stream, options) {
                Ok(v) => value.push(v),
                Err(err) => return Err(err),
            };

            if stream.curr != ">" {
                break;
            }
        }

        return Ok(value);
    }
}

impl Render for BlockQuote {
    fn render_into(
        &self,
        writer: &mut dyn fmt::Write,
        options: &RenderOptions,
    ) -> Result<(), fmt::Error> {
        return self.to_html().render_into(writer, options);
    }
}

impl html::ToHtml for BlockQuote {
    fn to_html(&self) -> html::Node {
        let mut el = html::Element::new("blockquote");

        for child in self.0.iter() {
            el.push(child.to_html());
        }

        return el.to_html();
    }
}

impl fmt::Display for BlockQuote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return self.render_into(f, &RenderOptions::default());
    }
}
