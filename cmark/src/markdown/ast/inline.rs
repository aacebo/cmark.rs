use std::fmt;

use crate::{ParseError, ParseOptions, Render, RenderOptions, Revert, Stream, html};

#[derive(Debug, Clone)]
pub enum Inline {
    Bold(super::Bold),
    BreakLine(super::BreakLine),
    Text(super::Text),
}

impl Inline {
    pub fn parse(stream: &mut Stream, options: &ParseOptions) -> Result<Self, ParseError> {
        if stream.is_eof() {
            return Err(stream.eof());
        }

        log::debug!(target: "cmark:md:inline", "parse");
        let mut copy = stream.clone();

        if let Ok(v) = super::Bold::parse(stream, options) {
            return Ok(Self::Bold(v));
        }

        stream.revert(&mut copy);

        if let Ok(v) = super::BreakLine::parse(stream, options) {
            return Ok(Self::BreakLine(v));
        }

        stream.revert(&mut copy);

        return match super::Text::parse(stream, options) {
            Ok(v) => Ok(Self::Text(v)),
            Err(err) => Err(err),
        };
    }
}

impl Render for Inline {
    fn render_into(
        &self,
        writer: &mut dyn fmt::Write,
        options: &RenderOptions,
    ) -> Result<(), fmt::Error> {
        return match self {
            Self::Bold(v) => v.render_into(writer, options),
            Self::BreakLine(v) => v.render_into(writer, options),
            Self::Text(v) => v.render_into(writer, options),
        };
    }
}

impl html::ToHtml for Inline {
    fn to_html(&self) -> html::Node {
        return match self {
            Self::Bold(v) => v.to_html(),
            Self::BreakLine(v) => v.to_html(),
            Self::Text(v) => v.to_html(),
        };
    }
}

impl fmt::Display for Inline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return self.render_into(f, &RenderOptions::default());
    }
}
