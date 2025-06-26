use super::token::*;

#[derive(Debug, Clone)]
pub struct TokenPointer {
    pub prev: Token,
    pub curr: Token,
}

impl TokenPointer {
    pub fn next(&mut self, token: Token) {
        self.prev = self.curr.clone();
        self.curr = token;
    }
}

impl Default for TokenPointer {
    fn default() -> Self {
        return Self {
            prev: Token::default(),
            curr: Token::default(),
        };
    }
}
