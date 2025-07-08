use crate::tokens::Token;

#[derive(Debug, Clone)]
pub struct Iter {
    pub prev: Token,
    pub curr: Token,
}

impl Iter {
    pub fn next(&mut self, token: Token) {
        self.prev = self.curr.clone();
        self.curr = token;
    }
}

impl Default for Iter {
    fn default() -> Self {
        return Self {
            prev: Token::default(),
            curr: Token::default(),
        };
    }
}
