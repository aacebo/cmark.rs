use crate::Position;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub start: Position,
    pub end: Position,
    pub message: String,
}

impl ParseError {
    pub fn from_str(start: Position, end: Position, message: &str) -> Self {
        return Self {
            start,
            end,
            message: message.to_string(),
        };
    }

    pub fn from_string(start: Position, end: Position, message: String) -> Self {
        return Self {
            start,
            end,
            message,
        };
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "[{}:{}] => {}",
            self.start.ln + 1,
            self.start.col + 1,
            self.message
        );
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return Some(self);
    }
}
