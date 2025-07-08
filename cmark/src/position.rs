use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Position {
    pub path: Option<&'static str>,
    pub ln: usize,
    pub col: usize,
    pub index: usize,
}

impl Position {
    pub fn new() -> Self {
        return Self {
            path: None,
            ln: 0,
            col: 0,
            index: 0,
        };
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let value = match self.path {
            Some(path) => format!("{}\n\t{}:{}", path, self.ln + 1, self.col + 1),
            None => format!("{}:{}", self.ln + 1, self.col + 1),
        };

        return write!(f, "{}", value);
    }
}