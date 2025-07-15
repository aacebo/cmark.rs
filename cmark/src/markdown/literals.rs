use std::fmt;

use crate::{Cursor, ParseToken, Position, Token};

macro_rules! define_literal_tokens {
    ($($tokens:literal $name:ident $method:ident)*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

            $(
                pub fn $method(&self) -> bool {
                    return false;
                }
            )*
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

        impl ParseToken for Literal {
            fn parse(cursor: &mut Cursor) -> Option<Token> {
                if cursor.is_eof() {
                    return None;
                }

                $(
                    match $name::parse(cursor) {
                        None => {},
                        Some(token) => return Some(token),
                    };
                )*

                return None;
            }
        }

        $(
            #[doc = $tokens]
            #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
            pub struct $name {
                pub start: Position,
                pub end: Position,
            }

            impl $name {
                pub fn new(start: Position, end: Position) -> Self {
                    return Self { start, end };
                }

                pub fn kind() -> &'static str {
                    return $tokens;
                }

                pub fn as_str(&self) -> &str {
                    return $tokens;
                }

                pub fn as_bytes(&self) -> &[u8] {
                    return self.as_str().as_bytes();
                }
            }

            impl ParseToken for $name {
                fn parse(cursor: &mut Cursor) -> Option<Token> {
                    if cursor.is_eof() {
                        return None;
                    }

                    if !cursor.next_if($tokens) {
                        return None;
                    }

                    return Some(
                        Token::Markdown(
                            super::MdToken::Literal(
                                Literal::$name(Self::new(cursor.start, cursor.end))
                            )
                        )
                    );
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
    "\n" NewLine is_newline
    " " Space is_space
    "\t" Tab is_tab
    ":" Colon is_colon
    "!" Bang is_bang
    "#" Hash is_hash
    "@" At is_at
    "[" LeftBracket is_left_bracket
    "]" RightBracket is_right_bracket
    "(" LeftParen is_left_paren
    ")" RightParen is_right_paren
    "{" LeftBrace is_left_brace
    "}" RightBrace is_right_brace
    "*" Asterisk is_asterisk
    "+" Plus is_plus
    "%" Percent is_percent
    "-" Dash is_dash
    "_" Underscore is_underscore
    "~" Tilde is_tilde
    "=" Equals is_equals
    "==" EqualsEquals is_equals_equals
    "!=" NotEquals is_not_equals
    ">" GreaterThan is_greater_than
    ">=" GreaterThanEquals is_greater_than_equals
    "<" LessThan is_less_than
    "<=" LessThanEquals is_less_than_equals
    "'" Quote is_quote
    "\"" DoubleQuote is_double_quote
    "`" BackQuote is_back_quote
    "." Period is_period
    "|" Pipe is_pipe
    "||" Or is_or
    "&" Ampersand is_ampersand
    "&&" And is_and
    "/" Slash is_slash
    "\\" BackSlash is_back_slash
    "true" True is_true
    "false" False is_false
}
