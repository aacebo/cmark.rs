use crate::ParseError;

use super::*;

pub trait Parse {
    fn parse<'a>(stream: &mut Stream) -> Result<Node<'a>, ParseError>;
}
