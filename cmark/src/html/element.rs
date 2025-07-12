use crate::{
    Render,
    html::{Attributes, Classes, Node},
};

#[derive(Debug, Clone)]
pub struct Element {
    pub selector: String,
    pub attributes: Attributes,
    pub children: Vec<Node>,
}

impl Element {
    pub fn new(selector: &str) -> Self {
        return Self {
            selector: selector.to_string(),
            attributes: Attributes::new(),
            children: vec![],
        };
    }

    pub fn has_attr(&self, name: &str) -> bool {
        return self.attributes.has(name.to_string());
    }

    pub fn get_attr(&self, name: &str) -> Option<String> {
        let value = self.attributes.try_get(name.to_string());
        return match value {
            Some(v) => Some(v.to_string()),
            None => None,
        };
    }

    pub fn set_attr(&mut self, name: &str, value: String) {
        self.attributes.put(name.to_string(), value);
    }

    pub fn del_attr(&mut self, name: &str) {
        self.attributes.del(name.to_string());
    }

    pub fn has_class(&self, name: &str) -> bool {
        let classes = match self.attributes.try_get(String::from("class")) {
            None => Classes::new(),
            Some(value) => Classes::from(value.to_string()),
        };

        return classes.contains(name);
    }

    pub fn add_class(&mut self, name: &str) -> bool {
        let mut classes = match self.attributes.try_get(String::from("class")) {
            None => Classes::new(),
            Some(value) => Classes::from(value.to_string()),
        };

        if !classes.add(name) {
            return false;
        }

        self.attributes
            .put(String::from("class"), classes.to_string());
        return true;
    }

    pub fn del_class(&mut self, name: &str) -> bool {
        let mut classes = match self.attributes.try_get(String::from("class")) {
            None => Classes::new(),
            Some(value) => Classes::from(value.to_string()),
        };

        let added = classes.remove(name);
        self.attributes
            .put(String::from("class"), classes.to_string());
        return added;
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
}

impl Render for Element {
    fn render_into(&self, writer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        write!(writer, "<{}", self.selector)?;
        self.attributes.render_into(writer)?;

        if self.children.is_empty() {
            return Ok(write!(writer, " />").unwrap());
        }

        self.children.render_into(writer)?;
        write!(writer, "</{}>", self.selector)?;
        return Ok(());
    }
}
