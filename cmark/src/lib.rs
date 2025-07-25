mod cursor;
pub use cursor::*;

mod parse_error;
pub use parse_error::*;

mod position;
pub use position::*;

mod render;
pub use render::*;

mod revert;
use revert::*;

mod options;
pub use options::*;

mod parser;
pub use parser::*;

mod stream;
pub use stream::*;

mod token;
pub use token::*;

use crate::{html::ToHtml, markdown::ast};

pub mod html;
pub mod markdown;

#[allow(unused_imports)]
#[cfg(feature = "rsx")]
pub use cmark_rsx::*;

pub fn parse(src: Vec<u8>, options: &ParseOptions) -> Result {
    let mut stream = Stream::from(src);
    let mut el = html::Fragment::new();

    while !stream.curr.is_eof() {
        match ast::Node::parse_block(&mut stream, options) {
            Ok(node) => {
                el.push(node.clone().to_html());
            }
            Err(err) => {
                if err.is_eof() {
                    break;
                }

                if err.is_ignore() {
                    continue;
                }

                return Err(err);
            }
        };
    }

    return Ok(el.to_html());
}
