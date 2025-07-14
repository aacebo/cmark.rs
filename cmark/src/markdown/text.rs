use std::fmt;

use crate::{Cursor, ParseToken, Position, Token, markdown::MdToken};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Text {
    pub start: Position,
    pub end: Position,
    pub data: Box<[u8]>,
}

impl Text {
    pub fn new(start: Position, end: Position, data: Box<[u8]>) -> Self {
        return Self { start, end, data };
    }

    pub fn as_str(&self) -> &str {
        return std::str::from_utf8(&self.data).unwrap();
    }

    pub fn as_bytes(&self) -> &[u8] {
        return self.as_str().as_bytes();
    }
}

impl ParseToken for Text {
    fn parse(cursor: &mut Cursor) -> Option<Token> {
        cursor.next_while_alpha();

        let value = match cursor.to_str() {
            Ok(v) => v,
            Err(_) => return None,
        };

        return Some(Token::Markdown(MdToken::Text(Self::new(
            cursor.start,
            cursor.end,
            value.as_bytes().into(),
        ))));
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.as_str());
    }
}
