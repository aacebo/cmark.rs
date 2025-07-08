use crate::{cursor::Cursor, parse_error::ParseError, tokens::Token};

pub trait Scan {
    fn scan(&self, cursor: &mut Cursor) -> Result<Token, ParseError>;
}
