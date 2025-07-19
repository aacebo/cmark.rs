use crate::html::{Element, query::Rule};

#[derive(Debug, Clone)]
pub struct Or {
    items: Vec<Rule>,
}

impl Or {
    pub fn new() -> Self {
        return Self { items: vec![] };
    }

    pub fn eval(&self, node: &Element) -> bool {
        for rule in self.items.iter() {
            if rule.eval(node) {
                return true;
            }
        }

        return false;
    }

    pub fn to_rule(&self) -> Rule {
        return Rule::Or(self.clone());
    }
}

#[macro_export]
macro_rules! or {
    ($($query:expr)*) => {{
        let mut q = $crate::html::query::Or::new();
        $(q.items.push($query);)*
        q
    }};
}
