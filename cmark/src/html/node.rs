use std::{fmt, sync::Arc};

use crate::{
    Render, RenderOptions,
    html::{Element, Fragment, Raw},
};

pub trait ToHtml {
    fn to_html(&self) -> Node;
}

#[derive(Debug, Clone)]
pub enum Node {
    Elem(Element),
    Frag(Fragment),
    Raw(Raw),
    Other(Arc<dyn Render>),
}

impl From<Element> for Node {
    fn from(value: Element) -> Self {
        return Self::Elem(value);
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
    fn render_into(
        &self,
        writer: &mut dyn fmt::Write,
        options: &RenderOptions,
    ) -> Result<(), fmt::Error> {
        return match self {
            Self::Elem(node) => node.render_into(writer, options),
            Self::Frag(node) => node.render_into(writer, options),
            Self::Raw(node) => node.render_into(writer, options),
            Self::Other(node) => node.render_into(writer, options),
        };
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        return match self {
            Self::Elem(node) => {
                node.render(&RenderOptions::default()) == other.render(&RenderOptions::default())
            }
            Self::Frag(node) => {
                node.render(&RenderOptions::default()) == other.render(&RenderOptions::default())
            }
            Self::Raw(node) => {
                node.render(&RenderOptions::default()) == other.render(&RenderOptions::default())
            }
            Self::Other(node) => {
                node.render(&RenderOptions::default()) == other.render(&RenderOptions::default())
            }
        };
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return self.render_into(f, &RenderOptions::default());
    }
}
