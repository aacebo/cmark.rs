use std::fmt::{Display, Pointer};

#[derive(Debug)]
pub struct Group<T, E> {
    results: Vec<Result<T, E>>,
}

impl<T, E> Group<T, E> {
    pub fn new() -> Self {
        return Self { results: vec![] };
    }

    pub fn len(&self) -> usize {
        return self.results.len();
    }

    pub fn push(&mut self, result: Result<T, E>) {
        self.results.push(result);
    }

    pub fn pop(&mut self) -> Option<Result<T, E>> {
        return self.results.pop();
    }
}

impl<T, E> From<Vec<Result<T, E>>> for Group<T, E> {
    fn from(value: Vec<Result<T, E>>) -> Self {
        return Self { results: value };
    }
}

impl<T: Clone, E: Clone> From<&[Result<T, E>]> for Group<T, E> {
    fn from(value: &[Result<T, E>]) -> Self {
        let mut results: Vec<Result<T, E>> = vec![];

        for result in value.iter() {
            results.push(result.clone());
        }

        return Self { results };
    }
}

impl<T, E> Display for Group<T, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for result in self.results.iter() {
            result.fmt(f)?;
        }

        return Ok(());
    }
}
