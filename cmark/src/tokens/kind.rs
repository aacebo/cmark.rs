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

impl ToString for Kind {
    fn to_string(&self) -> String {
        return match self {
            Kind::Eof => String::from("eof"),
        };
    }
}

impl From<u8> for Kind {
    fn from(value: u8) -> Self {
        return match value {
            _ => Kind::Eof,
        };
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