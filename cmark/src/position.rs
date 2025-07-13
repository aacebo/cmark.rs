use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Position {
    pub ln: usize,
    pub col: usize,
    pub index: usize,
}

impl Position {
    pub fn new() -> Self {
        return Self {
            ln: 0,
            col: 0,
            index: 0,
        };
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.index.cmp(&other.index);
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        return write!(f, "{}:{}", self.ln + 1, self.col + 1);
    }
}
