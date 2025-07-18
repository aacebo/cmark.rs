use crate::{Render, RenderOptions, html::Node};

#[derive(Debug, Clone)]
pub struct Fragment {
    pub children: Vec<Node>,
}

impl Fragment {
    pub fn new() -> Self {
        return Self { children: vec![] };
    }

    pub fn count(&self) -> usize {
        return self.children.len();
    }

    pub fn push(&mut self, element: Node) {
        self.children.push(element);
    }

    pub fn pop(&mut self) -> Option<Node> {
        return self.children.pop();
    }

    pub fn to_html(&self) -> Node {
        return Node::Frag(self.clone());
    }
}

impl Render for Fragment {
    fn render_into(
        &self,
        writer: &mut dyn std::fmt::Write,
        options: &RenderOptions,
    ) -> Result<(), std::fmt::Error> {
        self.children.render_into(writer, options)?;
        return Ok(());
    }
}
