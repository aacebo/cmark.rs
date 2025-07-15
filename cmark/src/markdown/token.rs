use std::fmt;

use crate::{Cursor, ParseToken, Position, Token};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MdToken {
    Decimal(super::Decimal),
    Int(super::Int),
    Text(super::Text),
    Literal(super::Literal),
}

impl MdToken {
    pub fn start(&self) -> Position {
        return match self {
            Self::Decimal(v) => v.start,
            Self::Int(v) => v.start,
            Self::Text(v) => v.start,
            Self::Literal(v) => v.start(),
        };
    }

    pub fn end(&self) -> Position {
        return match self {
            Self::Decimal(v) => v.end,
            Self::Int(v) => v.end,
            Self::Text(v) => v.end,
            Self::Literal(v) => v.end(),
        };
    }

    pub fn as_str(&self) -> &str {
        return match self {
            Self::Decimal(v) => v.as_str(),
            Self::Int(v) => v.as_str(),
            Self::Text(v) => v.as_str(),
            Self::Literal(v) => v.as_str(),
        };
    }

    pub fn as_bytes(&self) -> &[u8] {
        return self.as_str().as_bytes();
    }
}

impl fmt::Display for MdToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Self::Decimal(v) => write!(f, "{}", v),
            Self::Int(v) => write!(f, "{}", v),
            Self::Text(v) => write!(f, "{}", v),
            Self::Literal(v) => write!(f, "{}", v),
        };
    }
}

impl PartialEq<&str> for MdToken {
    fn eq(&self, other: &&str) -> bool {
        return self.as_str() == *other;
    }
}

impl ParseToken for MdToken {
    fn parse(cursor: &mut Cursor) -> Option<Token> {
        if cursor.is_eof() {
            return None;
        }

        match super::Literal::parse(cursor) {
            None => {}
            Some(token) => return Some(token),
        };

        match super::Decimal::parse(cursor) {
            None => {}
            Some(token) => return Some(token),
        };

        match super::Int::parse(cursor) {
            None => {}
            Some(token) => return Some(token),
        };

        return super::Text::parse(cursor);
    }
}

#[macro_export]
macro_rules! md_token {
    [newline] => { $crate::markdown::NewLine };
    [space] => { $crate::markdown::Space };
    [tab] => { $crate::markdown::Tab };
    [:] => { $crate::markdown::Colon };
    [!] => { $crate::markdown::Bang };
    [#] => { $crate::markdown::Hash };
    [@] => { $crate::markdown::At };
    ['['] => { $crate::markdown::LeftBracket };
    [']'] => { $crate::markdown::RightBracket };
    ['('] => { $crate::markdown::LeftParen };
    [')'] => { $crate::markdown::RightParen };
    ['{'] => { $crate::markdown::LeftBrace };
    ['}'] => { $crate::markdown::RightBrace };
    [*] => { $crate::markdown::Asterisk };
    [+] => { $crate::markdown::Plus };
    [%] => { $crate::markdown::Percent };
    [-] => { $crate::markdown::Dash };
    [_] => { $crate::markdown::Underscore };
    [~] => { $crate::markdown::Tilde };
    [=] => { $crate::markdown::Equals };
    [==] => { $crate::markdown::EqualsEquals };
    [!=] => { $crate::markdown::NotEquals };
    [>] => { $crate::markdown::GreaterThan };
    [>=] => { $crate::markdown::GreaterThanEquals };
    [<] => { $crate::markdown::LessThan };
    [<=] => { $crate::markdown::LessThanEquals };
    ['\''] => { $crate::markdown::Quote };
    ['"'] => { $crate::markdown::DoubleQuote };
    ['`'] => { $crate::markdown::BackQuote };
    [.] => { $crate::markdown::Period };
    [|] => { $crate::markdown::Pipe };
    [||] => { $crate::markdown::Or };
    [&] => { $crate::markdown::Ampersand };
    [&&] => { $crate::markdown::And };
    [/] => { $crate::markdown::Slash };
    ['\\'] => { $crate::markdown::BackSlash };

    [text] => { $crate::markdown::Text };
    [int] => { $crate::markdown::Int };
    [decimal] => { $crate::markdown::Decimal };
    [true] => { $crate::markdown::True };
    [false] => { $crate::markdown::False };
}
