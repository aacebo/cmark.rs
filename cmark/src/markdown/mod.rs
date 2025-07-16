pub mod ast;

mod token;
pub use token::*;

mod text;
pub use text::*;

mod decimal;
pub use decimal::*;

mod int;
pub use int::*;

mod literals;
pub use literals::*;

use crate::{Extension, ParseOptions, Result, Stream, html::ToHtml};

#[derive(Debug, Clone, Copy)]
pub struct Markdown;

impl Markdown {
    pub fn new() -> Self {
        return Self;
    }
}

impl Extension for Markdown {
    fn name(&self) -> &str {
        return "markdown";
    }

    fn parse_block(&mut self, stream: &mut Stream, options: &ParseOptions) -> Result {
        if stream.cursor().is_eof() {
            return Err(stream.eof());
        }

        let node = ast::Block::parse(stream, options)?;
        return Ok(node.to_html());
    }

    fn parse_inline(&mut self, stream: &mut Stream, options: &ParseOptions) -> Result {
        if stream.cursor().is_eof() {
            return Err(stream.eof());
        }

        let node = ast::Inline::parse(stream, options)?;
        return Ok(node.to_html());
    }
}
