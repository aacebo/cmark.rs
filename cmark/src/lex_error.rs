use super::position::Position;

#[derive(Debug, Clone)]
pub struct LexError {
    pub start: Position,
    pub end: Position,
    pub message: String,
}

impl LexError {
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

impl std::fmt::Display for LexError {
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

impl std::error::Error for LexError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return Some(self);
    }
}
