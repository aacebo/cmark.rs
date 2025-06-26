use crate::HTMLElement;

pub struct Fragment {
    pub children: Vec<Box<dyn HTMLElement>>,
}

impl Fragment {
    pub fn new() -> Self {
        return Self { children: vec![] };
    }

    pub fn from(children: Vec<Box<dyn HTMLElement>>) -> Self {
        return Self { children };
    }
}

impl HTMLElement for Fragment {
    fn render_into(&self, writer: &mut dyn std::fmt::Write) -> std::fmt::Result {
        for child in self.children.iter() {
            child.render_into(writer)?;
        }

        return Ok(());
    }
}
