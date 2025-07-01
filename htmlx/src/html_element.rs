use crate::{Attributes, Element};

#[derive(Debug, Clone)]
pub struct HTMLElement<'a, T: Element> {
    pub selector: &'a str,
    pub attributes: Attributes<'a>,
    pub content: Option<T>,
}

impl<'a, T: Element> HTMLElement<'a, T> {
    pub fn new(selector: &'a str) -> Self {
        return Self {
            selector,
            attributes: Attributes::new(),
            content: None,
        };
    }
}

impl<'a, T: Element> Element for HTMLElement<'a, T> {
    fn render_into<W: std::fmt::Write>(self, writer: &mut W) -> std::fmt::Result {
        return match self.content {
            None => {
                write!(writer, "<{}", self.selector)?;
                self.attributes.render_into(writer)?;
                write!(writer, "/>")
            }
            Some(content) => {
                write!(writer, "<{}", self.selector)?;
                self.attributes.render_into(writer)?;
                write!(writer, ">")?;
                content.render_into(writer)?;
                write!(writer, "</{}>", self.selector)
            }
        };
    }
}
