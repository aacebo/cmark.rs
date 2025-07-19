use crate::html::{Element, query::Rule};

#[derive(Debug, Clone)]
pub struct AttrEq {
    key: String,
    value: String,
}

impl AttrEq {
    pub fn new(key: &str, value: &str) -> Self {
        return Self {
            key: key.to_string(),
            value: value.to_string(),
        };
    }

    pub fn eval(&self, node: &Element) -> bool {
        return match node.get_attr(self.key.as_str()) {
            Some(value) => value == self.value,
            None => false,
        };
    }

    pub fn to_rule(&self) -> Rule {
        return Rule::AttrEq(self.clone());
    }
}

#[macro_export]
macro_rules! attr_eq {
    ($key:ident, $value:literal) => {
        $crate::html::query::AttrEq::new(stringify!($key), $value)
    };
}
