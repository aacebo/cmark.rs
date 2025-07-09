use crate::{
    Render,
    html::{Attributes, Classes, Node},
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

    pub fn set_attr(&mut self, name: &'a str, value: String) {
        self.attributes.put(name, value);
    }

    pub fn del_attr(&mut self, name: &'a str) {
        self.attributes.del(name);
    }

    pub fn has_class(&self, name: &str) -> bool {
        let classes = match self.attributes.try_get("class") {
            None => Classes::new(),
            Some(value) => Classes::from(value.to_string()),
        };

        return classes.contains(name);
    }

    pub fn add_class(&mut self, name: &str) -> bool {
        let mut classes = match self.attributes.try_get("class") {
            None => Classes::new(),
            Some(value) => Classes::from(value.to_string()),
        };

        if !classes.add(name) {
            return false;
        }

        self.attributes.put("class", classes.to_string());
        return true;
    }

    pub fn del_class(&mut self, name: &str) -> bool {
        let mut classes = match self.attributes.try_get("class") {
            None => Classes::new(),
            Some(value) => Classes::from(value.to_string()),
        };

        let added = classes.remove(name);
        self.attributes.put("class", classes.to_string());
        return added;
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
