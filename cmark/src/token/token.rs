use crate::{error::Error, position::Position};

#[derive(Debug, Clone, Default)]
pub struct Token {
    pub kind: u8,
    pub start: Position,
    pub end: Position,
    pub value: Vec<u8>,
}

impl Token {
    pub fn new(kind: u8, start: Position, end: Position, value: Vec<u8>) -> Self {
        return Self {
            kind,
            start,
            end,
            value,
        };
    }

    pub fn to_error(&self, message: &str) -> Error {
        return Error::from_str(self.start, self.end, message);
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        return String::from_utf8(self.value.clone()).unwrap();
    }
}
