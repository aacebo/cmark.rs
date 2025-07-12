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
    Literal(Literal),
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

macro_rules! define_literal_tokens {
    ($($tokens:literal pub struct $name:ident)*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Hash)]
        pub enum Literal {
            $($name($name), )*
        }

        impl Literal {
            pub fn start(&self) -> Position {
                return match self {
                    $(Self::$name(v) => v.start, )*
                };
            }

            pub fn end(&self) -> Position {
                return match self {
                    $(Self::$name(v) => v.end, )*
                };
            }

            pub fn as_str(&self) -> &str {
                return match self {
                    $(Self::$name(v) => v.as_str(), )*
                };
            }

            pub fn as_bytes(&self) -> &[u8] {
                return self.as_str().as_bytes();
            }
        }

        impl fmt::Display for Literal {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                return match self {
                    $(Self::$name(v) => write!(f, "{}", v.as_str()), )*
                };
            }
        }

        impl PartialEq<&str> for Literal {
            fn eq(&self, other: &&str) -> bool {
                return self.as_str() == *other;
            }
        }

        $(
            #[doc = $tokens]
            #[derive(Debug, Clone, Copy, Default, PartialEq, Hash)]
            pub struct $name {
                pub start: Position,
                pub end: Position,
            }

            impl $name {
                pub fn new(start: Position, end: Position) -> Self {
                    return Self { start, end };
                }

                pub fn as_str(&self) -> &str {
                    return stringify!($tokens);
                }

                pub fn as_bytes(&self) -> &[u8] {
                    return self.as_str().as_bytes();
                }
            }

            impl Parse for $name {
                fn parse(cursor: &mut Cursor) -> Option<Token> {
                    if !cursor.next_if(stringify!($tokens)) {
                        return None;
                    }

                    return Some(Token::Literal(Literal::$name(Self::new(cursor.start, cursor.end))));
                }
            }

            impl fmt::Display for $name {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    return write!(f, "{}", self.as_str());
                }
            }

            impl PartialEq<&str> for $name {
                fn eq(&self, other: &&str) -> bool {
                    return self.as_str() == *other;
                }
            }
        )*
    };
}

define_literal_tokens! {
    "\n" pub struct NewLine
    " " pub struct Space
    "\t" pub struct Tab
    ":" pub struct Colon
    "!" pub struct Bang
    "#" pub struct Hash
    "@" pub struct At
    "[" pub struct LeftBracket
    "]" pub struct RightBracket
    "(" pub struct LeftParen
    ")" pub struct RightParen
    "{" pub struct LeftBrace
    "}" pub struct RightBrace
    "*" pub struct Asterisk
    "+" pub struct Plus
    "%" pub struct Percent
    "-" pub struct Dash
    "_" pub struct Underscore
    "~" pub struct Tilde
    "=" pub struct Equals
    "==" pub struct EqualsEquals
    "!=" pub struct NotEquals
    ">" pub struct GreaterThan
    ">=" pub struct GreaterThanEquals
    "<" pub struct LessThan
    "<=" pub struct LessThanEquals
    "'" pub struct Quote
    "\"" pub struct DoubleQuote
    "`" pub struct BackQuote
    "." pub struct Period
    "|" pub struct Pipe
    "||" pub struct Or
    "&" pub struct Ampersand
    "&&" pub struct And
    "/" pub struct Slash
    "\\" pub struct BackSlash
    "true" pub struct True
    "false" pub struct False
}

#[macro_export]
macro_rules! token {
    ['\n'] => { $crate::tokens::NewLine };
    [' '] => { $crate::tokens::Space };
    ['\t'] => { $crate::tokens::Tab };
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
