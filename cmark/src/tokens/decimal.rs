use crate::{Cursor, Position, tokens::Parse};

#[derive(Debug, Clone, Copy, Default, PartialEq, Hash)]
pub struct Decimal<'a> {
    pub start: Position,
    pub end: Position,
    pub data: &'a [u8],
}

impl<'a> Decimal<'a> {
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

impl<'a> Parse for Decimal<'a> {
    fn parse(cursor: &'static mut Cursor) -> Option<Self> {
        if cursor.peek() < b'0' || cursor.peek() > b'9' {
            return None;
        }

        while cursor.peek() >= b'0' && cursor.peek() <= b'9' {
            cursor.next();
        }

        if cursor.peek() != b'.' {
            return None;
        }

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
