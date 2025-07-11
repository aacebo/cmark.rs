use std::sync::Arc;

use crate::tokens::Token;

#[derive(Clone, Debug, Default)]
pub struct Stream(Arc<Vec<Token>>);

impl Stream {
    pub fn new() -> Self {
        return Self {
            0: Arc::new(vec![]),
        };
    }

    pub fn is_empty(&self) -> bool {
        return self.0.is_empty();
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn get(&self, index: usize) -> Option<&Token> {
        return self.0.get(index);
    }

    pub fn iter(&self) -> Iter<'_> {
        return Iter::new(self);
    }

    pub fn push(&mut self, token: Token) {
        let tokens = Arc::make_mut(&mut self.0);
        tokens.push(token);
    }

    pub fn append(&mut self, stream: Stream) {
        let tokens = Arc::make_mut(&mut self.0);

        for token in stream.iter() {
            tokens.push(token.clone());
        }
    }
}

impl FromIterator<Token> for Stream {
    fn from_iter<I: IntoIterator<Item = Token>>(iter: I) -> Self {
        return Self {
            0: Arc::new(iter.into_iter().collect::<Vec<Token>>()),
        };
    }
}

impl Eq for Stream {}

impl PartialEq<Stream> for Stream {
    fn eq(&self, other: &Stream) -> bool {
        return self.iter().eq(other.iter());
    }
}

#[derive(Clone)]
pub struct Iter<'t> {
    stream: &'t Stream,
    index: usize,
}

impl<'t> Iter<'t> {
    fn new(stream: &'t Stream) -> Self {
        return Self { stream, index: 0 };
    }

    pub fn peek(&self) -> Option<&'t Token> {
        return self.stream.0.get(self.index);
    }
}

impl<'t> Iterator for Iter<'t> {
    type Item = &'t Token;

    fn next(&mut self) -> Option<&'t Token> {
        return self.stream.0.get(self.index).map(|tree| {
            self.index += 1;
            tree
        });
    }
}
