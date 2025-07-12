use std::fmt::{Result, Write};

use crate::{Render, html::Node};

/// Raw
///
/// raw html string
#[derive(Debug, Clone, Copy)]
pub struct Raw<'s>(&'s str);

impl<'a> Raw<'a> {
    pub fn to_node(&self) -> Node {
        return Node::Raw(self.clone());
    }
}

impl<'s> From<&'s str> for Raw<'s> {
    fn from(s: &'s str) -> Self {
        return Self(s);
    }
}

impl<'s> Render for Raw<'s> {
    fn render_into(&self, writer: &mut dyn Write) -> Result {
        return write!(writer, "{}", self.0);
    }
}

#[macro_export]
macro_rules! raw {
    ($text:expr) => {
        ::cmark::html::Raw::from($text)
    };
}
