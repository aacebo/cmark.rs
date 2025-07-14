use crate::{ParseOptions, Result};

pub trait Extension {
    fn name(&self) -> &str;
    fn parse(&self, options: &ParseOptions) -> Result;
    fn parse_block(&self, options: &ParseOptions) -> Result;
}
