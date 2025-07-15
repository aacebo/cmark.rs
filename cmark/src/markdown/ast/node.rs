use std::fmt;

use crate::{Render, html};

#[derive(Debug, Clone)]
pub enum Node {
    Block(super::Block),
    Inline(super::Inline),
}

impl Render for Node {
    fn render_into(&self, writer: &mut dyn fmt::Write) -> Result<(), fmt::Error> {
        return match self {
            Self::Block(v) => v.render_into(writer),
            Self::Inline(v) => v.render_into(writer),
        };
    }
}

impl html::ToHtml for Node {
    fn to_html(&self) -> html::Node {
        return match self {
            Self::Block(v) => v.to_html(),
            Self::Inline(v) => v.to_html(),
        };
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return self.render_into(f);
    }
}
