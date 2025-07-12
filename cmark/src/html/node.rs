use std::sync::Arc;

use crate::{
    Render,
    html::{Element, Raw},
};

#[derive(Debug, Clone)]
pub enum Node {
    Element(Element),
    Raw(Raw),
    Other(Arc<dyn Render>),
}

impl From<Element> for Node {
    fn from(value: Element) -> Self {
        return Self::Element(value);
    }
}

impl From<Raw> for Node {
    fn from(value: Raw) -> Self {
        return Self::Raw(value);
    }
}

impl From<Arc<dyn Render>> for Node {
    fn from(value: Arc<dyn Render>) -> Self {
        return Self::Other(value);
    }
}

impl Render for Node {
    fn render_into(&self, writer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        return match self {
            Self::Element(node) => node.render_into(writer),
            Self::Raw(node) => node.render_into(writer),
            Self::Other(node) => node.render_into(writer),
        };
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        return match self {
            Self::Element(node) => node.render() == other.render(),
            Self::Raw(node) => node.render() == other.render(),
            Self::Other(node) => node.render() == other.render(),
        };
    }
}
