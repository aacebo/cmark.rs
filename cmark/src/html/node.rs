use crate::{CMarkError, Render, html::Element};

#[derive(Debug, Clone)]
pub enum Node<'a> {
    Html(Element<'a>),
    Other(&'a dyn Render),
}

impl<'a> Render for Node<'a> {
    fn render_into(&self, writer: &mut dyn std::fmt::Write) -> Result<(), CMarkError> {
        return match self {
            Node::Html(node) => node.render_into(writer),
            Node::Other(node) => node.render_into(writer),
        };
    }
}
