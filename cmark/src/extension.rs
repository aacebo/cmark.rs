use crate::{ParseOptions, Result, Stream};

pub trait Extension {
    fn name(&self) -> &str;
    fn parse(&self, stream: &mut Stream, options: &ParseOptions) -> Result;
    fn parse_block(&self, stream: &mut Stream, options: &ParseOptions) -> Result;
}
