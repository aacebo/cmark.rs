use std::fmt;

use crate::{Cursor, tokens::Token};

/// A token in the lexer.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    /// End of file.
    Eof,
    /// Newline character.
    NewLine,
    /// Space character.
    Space,
    /// Tab character.
    Tab,
    /// Colon: `:`
    Colon,
    /// Exclamation mark: `!`
    Bang,
    /// Hash/pound sign: `#`
    Hash,
    /// At sign: `@`
    At,
    /// Left square bracket: `[`
    LeftBracket,
    /// Right square bracket: `]`
    RightBracket,
    /// Left parenthesis: `(`
    LeftParen,
    /// Right parenthesis: `)`
    RightParen,
    /// Left curly brace: `{`
    LeftBrace,
    /// Right curly brace: `}`
    RightBrace,
    /// Asterisk: `*`
    Asterisk,
    /// Plus sign: `+`
    Plus,
    /// Percent sign: `%`
    Percent,
    /// Dash or minus sign: `-`
    Dash,
    /// Underscore: `_`
    Underscore,
    /// Tilde: `~`
    Tilde,
    /// Equals sign: `=`
    Equals,
    /// Double equals: `==`
    EqualsEquals,
    /// Not equals: `!=`
    NotEquals,
    /// Greater than: `>`
    GreaterThan,
    /// Greater than or equal: `>=`
    GreaterThanEquals,
    /// Less than: `<`
    LessThan,
    /// Less than or equal: `<=`
    LessThanEquals,
    /// Single quote: `'`
    Quote,
    /// Double quote: `"`
    DoubleQuote,
    /// Backquote: `` ` ``
    BackQuote,
    /// Period or dot: `.`
    Period,
    /// Pipe: `|`
    Pipe,
    /// Logical OR: `||`
    Or,
    /// Ampersand: `&`
    Ampersand,
    /// Logical AND: `&&`
    And,
    /// Forward slash: `/`
    Slash,
    /// Backslash: `\`
    BackSlash,
    /// Integer literal, e.g., `123`
    Integer,
    /// Decimal number, e.g., `123.4`
    Decimal,
    /// Text literal, e.g., `hello`
    Text,
    /// Boolean literal: `true`
    True,
    /// Boolean literal: `false`
    False,
    /// Null literal: `null`
    Null,
}

impl Default for Kind {
    fn default() -> Self {
        return Self::Eof;
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
        match value {
            0 => Self::Eof,
            1 => Self::NewLine,
            2 => Self::Space,
            3 => Self::Tab,
            4 => Self::Colon,
            5 => Self::Bang,
            6 => Self::Hash,
            7 => Self::At,
            8 => Self::LeftBracket,
            9 => Self::RightBracket,
            10 => Self::LeftParen,
            11 => Self::RightParen,
            12 => Self::LeftBrace,
            13 => Self::RightBrace,
            14 => Self::Asterisk,
            15 => Self::Plus,
            16 => Self::Percent,
            17 => Self::Dash,
            18 => Self::Underscore,
            19 => Self::Tilde,
            20 => Self::Equals,
            21 => Self::EqualsEquals,
            22 => Self::NotEquals,
            23 => Self::GreaterThan,
            24 => Self::GreaterThanEquals,
            25 => Self::LessThan,
            26 => Self::LessThanEquals,
            27 => Self::Quote,
            28 => Self::DoubleQuote,
            29 => Self::BackQuote,
            30 => Self::Period,
            31 => Self::Pipe,
            32 => Self::Or,
            33 => Self::Ampersand,
            34 => Self::And,
            35 => Self::Slash,
            36 => Self::BackSlash,
            37 => Self::Integer,
            38 => Self::Decimal,
            39 => Self::Text,
            40 => Self::True,
            41 => Self::False,
            42 => Self::Null,
            v => panic!("value {} is not a valid token type", v),
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Eof => "eof",
            Self::NewLine => "new_line",
            Self::Space => "space",
            Self::Tab => "tab",
            Self::Colon => "colon",
            Self::Bang => "bang",
            Self::Hash => "hash",
            Self::At => "at",
            Self::LeftBracket => "left_bracket",
            Self::RightBracket => "right_bracket",
            Self::LeftParen => "left_paren",
            Self::RightParen => "right_paren",
            Self::LeftBrace => "left_brace",
            Self::RightBrace => "right_brace",
            Self::Asterisk => "asterisk",
            Self::Plus => "plus",
            Self::Percent => "percent",
            Self::Dash => "dash",
            Self::Underscore => "underscore",
            Self::Tilde => "tilde",
            Self::Equals => "equals",
            Self::EqualsEquals => "equals_equals",
            Self::NotEquals => "not_equals",
            Self::GreaterThan => "greater_than",
            Self::GreaterThanEquals => "greater_than_equals",
            Self::LessThan => "less_than",
            Self::LessThanEquals => "less_than_equals",
            Self::Quote => "quote",
            Self::DoubleQuote => "double_quote",
            Self::BackQuote => "back_quote",
            Self::Period => "period",
            Self::Pipe => "pipe",
            Self::Or => "or",
            Self::Ampersand => "ampersand",
            Self::And => "and",
            Self::Slash => "slash",
            Self::BackSlash => "back_slash",
            Self::Integer => "integer",
            Self::Decimal => "decimal",
            Self::Text => "text",
            Self::True => "true",
            Self::False => "false",
            Self::Null => "null",
        };

        return write!(f, "{name}");
    }
}

impl Kind {
    pub fn parse(&self, cursor: &mut Cursor) -> Option<Token> {
        return match self {
            Self::Eof => {
                if cursor.curr() != 0u8 {
                    return None;
                }

                return Some(cursor.create(Self::Eof));
            }
            Self::NewLine => {
                if cursor.curr() != b'\n' {
                    return None;
                }

                return Some(cursor.create(Self::Space));
            }
            Self::Space => {
                if cursor.curr() != b' ' {
                    return None;
                }

                return Some(cursor.create(Self::Space));
            }
            Self::Tab => {
                if cursor.curr() != b'\t' {
                    return None;
                }

                return Some(cursor.create(Self::Tab));
            }
            Self::Colon => {
                if cursor.curr() != b':' {
                    return None;
                }

                return Some(cursor.create(Self::Colon));
            }
            _ => None,
        };
    }
}
