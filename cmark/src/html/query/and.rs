use crate::html::{Element, query::Rule};

#[derive(Debug, Clone)]
pub struct And {
    items: Vec<Rule>,
}

impl And {
    pub fn new() -> Self {
        return Self { items: vec![] };
    }

    pub fn eval(&self, node: &Element) -> bool {
        for rule in self.items.iter() {
            if !rule.eval(node) {
                return false;
            }
        }

        return true;
    }

    pub fn to_rule(&self) -> Rule {
        return Rule::And(self.clone());
    }
}

#[macro_export]
macro_rules! and {
    ($($query:expr)*) => {{
        let mut q = $crate::html::query::And::new();
        $(q.items.push($query);)*
        q
    }};
}
