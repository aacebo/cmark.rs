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
            Self::Element(node) => node.render_into(writer),
            Self::Raw(node) => node.render_into(writer),
            Self::Other(node) => node.render_into(writer),
        };
    }
}

impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Self) -> bool {
        return match self {
            Self::Element(node) => node.render() == other.render(),
            Self::Raw(node) => node.render() == other.render(),
            Self::Other(node) => node.render() == other.render(),
        };
    }
}
