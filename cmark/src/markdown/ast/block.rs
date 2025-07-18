use std::fmt;

use crate::{ParseError, ParseOptions, Render, RenderOptions, Revert, Stream, html};

#[derive(Debug, Clone)]
pub enum Block {
    BlockQuote(super::BlockQuote),
    Paragraph(super::Paragraph),
}

impl Block {
    pub fn parse(stream: &mut Stream, options: &ParseOptions) -> Result<Self, ParseError> {
        if stream.is_eof() {
            return Err(stream.eof());
        }

        log::debug!(target: "cmark:md:block", "parse");
        let mut copy = stream.clone();

        if let Ok(v) = super::BlockQuote::parse(stream, options) {
            return Ok(Self::BlockQuote(v));
        }

        stream.revert(&mut copy);

        return match super::Paragraph::parse(stream, options) {
            Ok(v) => Ok(Self::Paragraph(v)),
            Err(err) => Err(err),
        };
    }
}

impl Render for Block {
    fn render_into(
        &self,
        writer: &mut dyn fmt::Write,
        options: &RenderOptions,
    ) -> Result<(), fmt::Error> {
        return match self {
            Self::BlockQuote(v) => v.render_into(writer, options),
            Self::Paragraph(v) => v.render_into(writer, options),
        };
    }
}

impl html::ToHtml for Block {
    fn to_html(&self) -> html::Node {
        return match self {
            Self::BlockQuote(v) => v.to_html(),
            Self::Paragraph(v) => v.to_html(),
        };
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return self.render_into(f, &RenderOptions::default());
    }
}
