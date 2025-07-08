use std::fmt::{Result, Write};

use crate::esc;

/// Element
///
/// any element that can be rendered
pub trait Element: Sized {
    fn render_into<W: Write>(&self, _writer: &mut W) -> Result {
        return Ok(());
    }

    fn render(&self) -> String {
        let mut buf = String::new();
        self.render_into(&mut buf).unwrap();
        return buf;
    }
}

impl Element for () {
    fn render_into<W: Write>(&self, _writer: &mut W) -> Result {
        return Ok(());
    }
}

impl<A: Element, B: Element> Element for (A, B) {
    fn render_into<W: Write>(&self, writer: &mut W) -> Result {
        self.0.render_into(writer)?;
        return self.1.render_into(writer);
    }
}

impl<A: Element, B: Element, C: Element> Element for (A, B, C) {
    fn render_into<W: Write>(&self, writer: &mut W) -> Result {
        self.0.render_into(writer)?;
        self.1.render_into(writer)?;
        return self.2.render_into(writer);
    }
}

impl<T: Element> Element for Option<T> {
    fn render_into<W: Write>(&self, writer: &mut W) -> Result {
        return match self {
            None => Ok(()),
            Some(x) => x.render_into(writer),
        };
    }
}

impl<T: Element> Element for Vec<T> {
    fn render_into<W: Write>(&self, writer: &mut W) -> Result {
        for elem in self {
            elem.render_into(writer)?;
        }

        return Ok(());
    }
}

impl<O: Element, E: Element> Element for std::result::Result<O, E> {
    fn render_into<W: Write>(&self, writer: &mut W) -> Result {
        return match self {
            Ok(o) => o.render_into(writer),
            Err(e) => e.render_into(writer),
        };
    }
}

impl Element for String {
    fn render_into<W: Write>(&self, writer: &mut W) -> Result {
        return esc(&self, writer);
    }
}

impl Element for &str {
    fn render_into<W: Write>(&self, writer: &mut W) -> Result {
        return esc(&self, writer);
    }
}

impl Element for std::borrow::Cow<'_, str> {
    fn render_into<W: Write>(&self, writer: &mut W) -> Result {
        return esc(&self, writer);
    }
}
