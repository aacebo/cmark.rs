use std::fmt;

use crate::{
    ParseError, ParseOptions, Render, Stream, html,
    markdown::ast::{Block, Inline},
};

#[derive(Debug, Clone)]
pub enum Node {
    Block(super::Block),
    Inline(super::Inline),
}

impl Node {
    pub fn parse_block(stream: &mut Stream, options: &ParseOptions) -> Result<Self, ParseError> {
        if stream.cursor().is_eof() {
            return Err(stream.eof());
        }

        let node = Block::parse(stream, options)?;
        return Ok(Self::Block(node));
    }

    pub fn parse_inline(stream: &mut Stream, options: &ParseOptions) -> Result<Self, ParseError> {
        if stream.cursor().is_eof() {
            return Err(stream.eof());
        }

        let node = Inline::parse(stream, options)?;
        return Ok(Self::Inline(node));
    }
}

impl Render for Node {
    fn render_into(&self, writer: &mut dyn fmt::Write) -> fmt::Result {
        return match self {
            Self::Block(v) => v.render_into(writer),
            Self::Inline(v) => v.render_into(writer),
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
        return self.render_into(f);
    }
}
