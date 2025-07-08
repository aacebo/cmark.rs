use std::{error::Error, fmt::Display};

use crate::ParseError;

#[derive(Debug, Clone)]
pub enum CMarkError {
    Parse(ParseError),
    Fmt(std::fmt::Error),
}

impl Display for CMarkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Parse(err) => write!(f, "{}", err),
            Self::Fmt(err) => write!(f, "{}", err),
        };
    }
}

impl Error for CMarkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        return match self {
            Self::Parse(err) => Some(err),
            Self::Fmt(err) => Some(err),
        };
    }
}

impl From<ParseError> for CMarkError {
    fn from(value: ParseError) -> Self {
        return Self::Parse(value);
    }
}

impl Into<ParseError> for CMarkError {
    fn into(self) -> ParseError {
        return match self {
            Self::Parse(err) => err,
            _ => panic!("error must be type `cmark::ParseError`"),
        };
    }
}

impl From<std::fmt::Error> for CMarkError {
    fn from(value: std::fmt::Error) -> Self {
        return Self::Fmt(value);
    }
}

impl Into<std::fmt::Error> for CMarkError {
    fn into(self) -> std::fmt::Error {
        return match self {
            Self::Fmt(err) => err,
            _ => panic!("error must be type `std::fmt::Error`"),
        };
    }
}
