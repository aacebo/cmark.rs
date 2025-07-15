use crate::{ParseOptions, Result, Stream};

pub trait Extension {
    fn name(&self) -> &str;
    fn parse_block(&mut self, stream: &mut Stream, options: &ParseOptions) -> Result;
    fn parse_inline(&mut self, stream: &mut Stream, options: &ParseOptions) -> Result;
}
