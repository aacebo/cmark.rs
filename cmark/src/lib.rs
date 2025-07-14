mod cursor;
pub use cursor::*;

mod iter;
pub use iter::*;

mod parse_error;
pub use parse_error::*;

mod position;
pub use position::*;

mod render;
pub use render::*;

mod revert;
pub use revert::*;

mod options;
pub use options::*;

mod parser;
pub use parser::*;

mod stream;
pub use stream::*;

mod token;
pub use token::*;

mod token_stream;
pub use token_stream::*;

pub mod html;
pub mod markdown;

///
/// Implementation
///

pub fn parse(_src: Vec<u8>, _options: &ParseOptions) -> Result {
    unimplemented!();
}

pub fn parse_block(_stream: &mut Stream, _options: &ParseOptions) -> Result {
    unimplemented!();
}

pub fn parse_inline(_stream: &mut Stream, _options: &ParseOptions) -> Result {
    unimplemented!();
}
