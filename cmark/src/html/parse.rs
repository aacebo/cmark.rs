use crate::ParseError;

use super::*;

pub trait Parse {
    fn parse<'a>(stream: Stream) -> Result<Node<'a>, ParseError>;
}
