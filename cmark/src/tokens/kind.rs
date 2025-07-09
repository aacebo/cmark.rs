use std::fmt::{Display, Formatter, Result};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Eof,
}

impl Default for Kind {
    fn default() -> Self {
        return Kind::Eof;
    }
}

impl Into<u8> for Kind {
    fn into(self) -> u8 {
        return self as u8;
    }
}

impl PartialEq<u8> for Kind {
    fn eq(&self, other: &u8) -> bool {
        return *self == Self::from(other.clone());
    }
}

impl From<u8> for Kind {
    fn from(value: u8) -> Self {
        return match value {
            _ => Kind::Eof,
        };
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let value = match self {
            Kind::Eof => "<eof>",
        };

        return write!(f, "{}", value);
    }
}
