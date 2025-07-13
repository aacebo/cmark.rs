use std::fmt;

use crate::{Cursor, Position};

pub trait Parse: Sized {
    fn parse(cursor: &mut Cursor) -> Option<Token>;
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Token {
    Invalid,
    Decimal(super::Decimal),
    Int(super::Int),
    Text(super::Text),
    Literal(super::Literal),
}

impl Token {
    pub fn start(&self) -> Position {
        return match self {
            Self::Invalid => Position::default(),
            Self::Decimal(v) => v.start,
            Self::Int(v) => v.start,
            Self::Text(v) => v.start,
            Self::Literal(v) => v.start(),
        };
    }

    pub fn end(&self) -> Position {
        return match self {
            Self::Invalid => Position::default(),
            Self::Decimal(v) => v.end,
            Self::Int(v) => v.end,
            Self::Text(v) => v.end,
            Self::Literal(v) => v.end(),
        };
    }

    pub fn as_str(&self) -> &str {
        return match self {
            Self::Invalid => "",
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Self::Invalid => write!(f, "<invalid>"),
            Self::Decimal(v) => write!(f, "{}", v),
            Self::Int(v) => write!(f, "{}", v),
            Self::Text(v) => write!(f, "{}", v),
            Self::Literal(v) => write!(f, "{}", v),
        };
    }
}

impl PartialEq<&str> for Token {
    fn eq(&self, other: &&str) -> bool {
        return self.as_str() == *other;
    }
}

impl Default for Token {
    fn default() -> Self {
        return Self::Invalid;
    }
}

impl Parse for Token {
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
macro_rules! token {
    [newline] => { $crate::tokens::NewLine };
    [space] => { $crate::tokens::Space };
    [tab] => { $crate::tokens::Tab };
    [:] => { $crate::tokens::Colon };
    [!] => { $crate::tokens::Bang };
    [#] => { $crate::tokens::Hash };
    [@] => { $crate::tokens::At };
    ['['] => { $crate::tokens::LeftBracket };
    [']'] => { $crate::tokens::RightBracket };
    ['('] => { $crate::tokens::LeftParen };
    [')'] => { $crate::tokens::RightParen };
    ['{'] => { $crate::tokens::LeftBrace };
    ['}'] => { $crate::tokens::RightBrace };
    [*] => { $crate::tokens::Asterisk };
    [+] => { $crate::tokens::Plus };
    [%] => { $crate::tokens::Percent };
    [-] => { $crate::tokens::Dash };
    [_] => { $crate::tokens::Underscore };
    [~] => { $crate::tokens::Tilde };
    [=] => { $crate::tokens::Equals };
    [==] => { $crate::tokens::EqualsEquals };
    [!=] => { $crate::tokens::NotEquals };
    [>] => { $crate::tokens::GreaterThan };
    [>=] => { $crate::tokens::GreaterThanEquals };
    [<] => { $crate::tokens::LessThan };
    [<=] => { $crate::tokens::LessThanEquals };
    ['\''] => { $crate::tokens::Quote };
    ['"'] => { $crate::tokens::DoubleQuote };
    ['`'] => { $crate::tokens::BackQuote };
    [.] => { $crate::tokens::Period };
    [|] => { $crate::tokens::Pipe };
    [||] => { $crate::tokens::Or };
    [&] => { $crate::tokens::Ampersand };
    [&&] => { $crate::tokens::And };
    [/] => { $crate::tokens::Slash };
    ['\\'] => { $crate::tokens::BackSlash };

    [text] => { $crate::tokens::Text };
    [int] => { $crate::tokens::Int };
    [decimal] => { $crate::tokens::Decimal };
    [true] => { $crate::tokens::True };
    [false] => { $crate::tokens::False };
}
