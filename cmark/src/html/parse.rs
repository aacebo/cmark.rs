use crate::ParseError;

use super::*;

pub trait Parse {
    fn parse(stream: &mut Stream) -> Result<Node, ParseError>;
}
