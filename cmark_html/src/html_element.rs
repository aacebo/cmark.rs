use std::fmt::{Result, Write};

use crate::escape;

/// Html Element
///
/// an html dom element that can be
/// rendered
pub trait HTMLElement {
    fn render(&self) -> String {
        let mut buf: String = String::new();
        self.render_into(&mut buf).unwrap();
        return buf;
    }

    fn render_into(&self, _writer: &mut dyn Write) -> Result;
}

impl HTMLElement for String {
    fn render_into(&self, writer: &mut dyn Write) -> Result {
        return escape(&self, writer);
    }
}

impl HTMLElement for &str {
    fn render_into(&self, writer: &mut dyn Write) -> Result {
        return escape(&self, writer);
    }
}

impl<T: HTMLElement> HTMLElement for Vec<T> {
    fn render_into(&self, writer: &mut dyn Write) -> Result {
        for elem in self {
            elem.render_into(writer)?;
        }

        return Ok(());
    }
}

impl<T: HTMLElement> HTMLElement for Option<T> {
    fn render_into(&self, writer: &mut dyn Write) -> Result {
        return match self {
            None => Ok(()),
            Some(x) => x.render_into(writer),
        };
    }
}

impl<T: HTMLElement, E: HTMLElement> HTMLElement for std::result::Result<T, E> {
    fn render_into(&self, writer: &mut dyn Write) -> Result {
        return match self {
            Ok(v) => v.render_into(writer),
            Err(e) => e.render_into(writer),
        };
    }
}
