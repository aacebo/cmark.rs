use crate::Element;

#[derive(Debug, Clone)]
pub struct Fragment<T: Element> {
    pub content: T,
}

impl<T: Element> Fragment<T> {
    pub fn new(content: T) -> Self {
        return Self { content };
    }
}

impl<T: Element> Element for Fragment<T> {
    fn render_into<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        return self.content.render_into(writer);
    }
}
