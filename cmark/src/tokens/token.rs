use common::errors::ToError;

use crate::{parse_error::ParseError, position::Position, tokens::Kind};

#[derive(Debug, Clone, Default)]
pub struct Token {
    pub kind: Kind,
    pub start: Position,
    pub end: Position,
    pub value: Vec<u8>,
}

impl Token {
    pub fn new(kind: Kind, start: Position, end: Position, value: Vec<u8>) -> Self {
        return Self {
            kind,
            start,
            end,
            value,
        };
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        return String::from_utf8(self.value.clone()).unwrap();
    }
}

impl ToError for Token {
    fn to_error(&self, message: &str) -> Box<dyn std::error::Error> {
        return Box::new(ParseError::from_str(self.start, self.end, message));
    }
}