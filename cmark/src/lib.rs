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

mod extension;
pub use extension::*;

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

use crate::html::ToHtml;

pub mod html;
pub mod markdown;

pub fn parse(src: Vec<u8>, options: &ParseOptions) -> Result {
    let mut stream = Stream::from(src);
    let mut el = html::Element::new("html");

    while stream.curr() != Token::Invalid {
        let node = parse_block(&mut stream, options)?;
        el.push(node);
    }

    return Ok(html::Node::Elem(el));
}

pub fn parse_block(stream: &mut Stream, options: &ParseOptions) -> Result {
    let node = markdown::ast::Block::parse(stream, options)?;
    return Ok(node.to_html());
}

pub fn parse_inline(stream: &mut Stream, options: &ParseOptions) -> Result {
    let node = markdown::ast::Inline::parse(stream, options)?;
    return Ok(node.to_html());
}
