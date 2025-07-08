use std::borrow::Cow;

use crate::{
    CMarkError, Render,
    html::{Attributes, Node},
};

#[derive(Debug, Clone)]
pub struct Element<'a> {
    pub selector: &'a str,
    pub attributes: Attributes<'a>,
    pub children: Vec<Node<'a>>,
}

impl<'a> Element<'a> {
    pub fn new(selector: &'a str) -> Self {
        return Self {
            selector,
            attributes: Attributes::new(),
            children: vec![],
        };
    }

    pub fn has_attr(&self, name: &str) -> bool {
        return self.attributes.has(name);
    }

    pub fn get_attr(&self, name: &str) -> Option<String> {
        let value = self.attributes.try_get(name);
        return match value {
            Some(v) => Some(v.to_string()),
            None => None,
        };
    }

    pub fn set_attr(&mut self, name: &'a str, value: &'a str) {
        self.attributes.put(name, Cow::from(value));
    }

    pub fn del_attr(&mut self, name: &'a str) {
        self.attributes.del(name);
    }

    pub fn count(&self) -> usize {
        return self.children.len();
    }

    pub fn push(&mut self, element: Node<'a>) {
        self.children.push(element);
    }

    pub fn pop(&mut self) -> Option<Node> {
        return self.children.pop();
    }
}

impl<'a> Render for Element<'a> {
    fn render_into(&self, writer: &mut dyn std::fmt::Write) -> Result<(), CMarkError> {
        write!(writer, "<{}", self.selector)?;
        self.attributes.render_into(writer)?;

        if self.children.is_empty() {
            return Ok(write!(writer, " />").unwrap());
        }

        for child in self.children.iter() {
            child.render_into(writer)?;
        }

        write!(writer, "</{}>", self.selector)?;
        return Ok(());
    }
}
