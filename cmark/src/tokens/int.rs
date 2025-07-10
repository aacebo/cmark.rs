use crate::{Cursor, Position, tokens::Parse};

#[derive(Debug, Clone, Copy, Default, PartialEq, Hash)]
pub struct Int<'a> {
    pub start: Position,
    pub end: Position,
    pub data: &'a [u8],
}

impl<'a> Int<'a> {
    pub fn new(start: Position, end: Position, data: &'a [u8]) -> Self {
        return Self { start, end, data };
    }

    pub fn as_str(&self) -> &str {
        return std::str::from_utf8(self.data).unwrap();
    }

    pub fn as_bytes(&self) -> &[u8] {
        return self.as_str().as_bytes();
    }
}

impl<'a> Parse for Int<'a> {
    fn parse(cursor: &'static mut Cursor) -> Option<Self> {
        if cursor.peek() < b'0' || cursor.peek() > b'9' {
            return None;
        }

        while cursor.peek() >= b'0' && cursor.peek() <= b'9' {
            cursor.next();
        }

        let value = match cursor.to_str() {
            Ok(v) => v,
            Err(_) => return None,
        };

        return Some(Self::new(cursor.start, cursor.end, value.as_bytes()));
    }
}
