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

impl ToString for Position {
    fn to_string(&self) -> String {
        return format!("{}:{}", self.ln + 1, self.col + 1);
    }
}