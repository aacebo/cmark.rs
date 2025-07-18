use std::fmt;

use crate::{
    ParseError, ParseOptions, Render, RenderOptions, Stream, html,
    markdown::ast::{Block, Inline},
};

#[derive(Debug, Clone)]
pub enum Node {
    Block(super::Block),
    Inline(super::Inline),
}

impl Node {
    pub fn parse_block(stream: &mut Stream, options: &ParseOptions) -> Result<Self, ParseError> {
        if stream.is_eof() {
            return Err(stream.eof());
        }

        return match Block::parse(stream, options) {
            Ok(node) => Ok(Self::Block(node)),
            Err(err) => Err(err),
        };
    }

    pub fn parse_inline(stream: &mut Stream, options: &ParseOptions) -> Result<Self, ParseError> {
        if stream.is_eof() {
            return Err(stream.eof());
        }

        return match Inline::parse(stream, options) {
            Ok(node) => Ok(Self::Inline(node)),
            Err(err) => Err(err),
        };
    }
}

impl Render for Node {
    fn render_into(&self, writer: &mut dyn fmt::Write, options: &RenderOptions) -> fmt::Result {
        return match self {
            Self::Block(v) => v.render_into(writer, options),
            Self::Inline(v) => v.render_into(writer, options),
        };
    }
}

impl html::ToHtml for Node {
    fn to_html(&self) -> html::Node {
        return match self {
            Self::Block(v) => v.to_html(),
            Self::Inline(v) => v.to_html(),
        };
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return self.render_into(f, &RenderOptions::default());
    }
}
