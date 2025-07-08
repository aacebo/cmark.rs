use crate::{cursor::Cursor, parse_error::ParseError, tokens::Token};

pub trait Parse {
    fn parse(&self, cursor: &mut Cursor) -> Result<Token, ParseError>;
}