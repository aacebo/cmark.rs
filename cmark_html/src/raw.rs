use std::fmt::{Result, Write};

use crate::HTMLElement;

/// A raw (unencoded) html string
#[derive(Debug, Clone)]
pub struct Raw(String);

impl From<&str> for Raw {
    fn from(s: &str) -> Self {
        return Raw(String::from(s));
    }
}

impl From<String> for Raw {
    fn from(value: String) -> Self {
        return Raw(value);
    }
}

impl HTMLElement for Raw {
    fn render_into(&self, writer: &mut dyn Write) -> Result {
        return write!(writer, "{}", self.0);
    }
}

#[macro_export]
macro_rules! raw {
    ($text:expr) => {
        ::cmark_html::Raw::from($text)
    };
}
