use crate::html::{
    Element,
    query::{And, AttrEq, Or},
};

#[derive(Debug, Clone)]
pub enum Rule {
    And(And),
    Or(Or),
    AttrEq(AttrEq),
}

impl Rule {
    pub fn eval(&self, node: &Element) -> bool {
        return match self {
            Self::And(v) => v.eval(node),
            Self::Or(v) => v.eval(node),
            Self::AttrEq(v) => v.eval(node),
        };
    }
}
