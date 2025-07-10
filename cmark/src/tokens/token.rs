use crate::{Cursor, Position};

pub trait Parse: Sized {
    fn parse(cursor: &'static mut Cursor) -> Option<Self>;
}

macro_rules! define_literal_token {
    ($($tokens:literal pub struct $name:ident)*) => {
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
                fn parse(cursor: &'static mut Cursor) -> Option<Self> {
                    let value = match cursor.to_str() {
                        Ok(v) => v,
                        Err(_) => return None,
                    };

                    if value != $tokens {
                        return None;
                    }

                    return Some(Self::new(cursor.start, cursor.end));
                }
            }
        )*
    };
}

define_literal_token! {
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
}

#[macro_export]
macro_rules! Token {
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
    [=] => {$crate::tokens::Equals };
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
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Token {}
