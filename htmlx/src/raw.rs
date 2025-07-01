use std::fmt::{Result, Write};

use crate::Element;

/// Raw
///
/// raw html string
#[derive(Debug)]
pub struct Raw<'s>(&'s str);

impl<'s> From<&'s str> for Raw<'s> {
    fn from(s: &'s str) -> Self {
        return Self(s);
    }
}

impl<'s> Element for Raw<'s> {
    fn render_into<W: Write>(self, writer: &mut W) -> Result {
        return write!(writer, "{}", self.0);
    }
}

#[macro_export]
macro_rules! raw {
    ($text:expr) => {
        ::htmlx::Raw::from($text)
    };
}
