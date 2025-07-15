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

use crate::{Extension, ParseOptions, Result, Stream};

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

    fn parse_block(&mut self, _stream: &mut Stream, _options: &ParseOptions) -> Result {
        unimplemented!()
    }

    fn parse_inline(&mut self, _stream: &mut Stream, _options: &ParseOptions) -> Result {
        unimplemented!()
    }
}
