use crate::Element;

#[derive(Debug, Clone)]
pub struct Fragment<T: Element> {
    pub children: T,
}

impl<T: Element> Fragment<T> {
    pub fn new(children: T) -> Self {
        return Self { children };
    }
}

impl<T: Element> Element for Fragment<T> {
    fn render_into<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        return self.children.render_into(writer);
    }
}
