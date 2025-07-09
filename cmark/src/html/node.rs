use crate::{
    Render,
    html::{Element, Raw},
};

#[derive(Debug, Clone)]
pub enum Node<'a> {
    Element(Element<'a>),
    Raw(Raw<'a>),
    Other(&'a dyn Render),
}

impl<'a> Render for Node<'a> {
    fn render_into(&self, writer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        return match self {
            Node::Element(node) => node.render_into(writer),
            Node::Raw(node) => node.render_into(writer),
            Node::Other(node) => node.render_into(writer),
        };
    }
}
