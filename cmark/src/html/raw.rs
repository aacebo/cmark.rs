use std::fmt;

use crate::{Render, html::Node};

/// Raw
///
/// raw html string
#[derive(Debug, Clone)]
pub struct Raw(String);

impl Raw {
    pub fn new() -> Self {
        return Self(String::from(""));
    }

    pub fn push(&mut self, value: &str) {
        self.0.push_str(value);
    }

    pub fn to_html(&self) -> Node {
        return Node::Raw(self.clone());
    }
}

impl From<&str> for Raw {
    fn from(value: &str) -> Self {
        return Self(value.to_string());
    }
}

impl From<String> for Raw {
    fn from(value: String) -> Self {
        return Self(value);
    }
}

impl Render for Raw {
    fn render_into(&self, writer: &mut dyn fmt::Write) -> fmt::Result {
        return write!(writer, "{}", self);
    }
}

impl fmt::Display for Raw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.0);
    }
}

impl PartialEq<&str> for Raw {
    fn eq(&self, other: &&str) -> bool {
        return self.0.as_str() == *other;
    }
}

#[macro_export]
macro_rules! raw {
    ($text:expr) => {
        ::cmark::html::Raw::from($text)
    };
}
