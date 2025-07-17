use std::fmt;

use crate::{Cursor, Position, markdown::MdToken};

pub trait ParseToken: Sized {
    fn parse(cursor: &mut Cursor) -> Option<Token>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Eof,
    Markdown(MdToken),
}

impl Default for Token {
    fn default() -> Self {
        return Self::Eof;
    }
}

impl Token {
    pub fn start(&self) -> Position {
        return match self {
            Self::Eof => Position::default(),
            Self::Markdown(v) => v.start(),
        };
    }

    pub fn end(&self) -> Position {
        return match self {
            Self::Eof => Position::default(),
            Self::Markdown(v) => v.end(),
        };
    }

    pub fn as_str(&self) -> &str {
        return match self {
            Self::Eof => "<eof>",
            Self::Markdown(v) => v.as_str(),
        };
    }

    pub fn as_bytes(&self) -> &[u8] {
        return self.as_str().as_bytes();
    }

    pub fn is_eof(&self) -> bool {
        return match self {
            Self::Eof => true,
            _ => false,
        };
    }

    pub fn is_markdown(&self) -> bool {
        return match self {
            Self::Markdown(_) => true,
            _ => false,
        };
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Self::Eof => write!(f, "<eof>"),
            Self::Markdown(v) => v.fmt(f),
        };
    }
}

impl PartialEq<&str> for Token {
    fn eq(&self, other: &&str) -> bool {
        return self.as_str() == *other;
    }
}

impl ParseToken for Token {
    fn parse(cursor: &mut Cursor) -> Option<Token> {
        if cursor.is_eof() {
            return Some(Self::Eof);
        }

        return MdToken::parse(cursor);
    }
}

#[macro_export]
macro_rules! token {
    [md $($tokens:tt)*] => { $crate::md_token![$($tokens)*] };
}
