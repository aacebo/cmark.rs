use crate::{ParseOptions, Result, Stream};

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

///
/// Implementation
///

pub fn parse_block(_stream: &mut Stream, _options: &ParseOptions) -> Result {
    unimplemented!();
}

pub fn parse_inline(_stream: &mut Stream, _options: &ParseOptions) -> Result {
    unimplemented!();
}
