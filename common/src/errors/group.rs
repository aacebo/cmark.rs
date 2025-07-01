use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug, Clone)]
pub struct Group<T: Error> {
    errors: Vec<T>,
}

impl<T: Error> Group<T> {
    pub fn new() -> Self {
        return Self { errors: vec![] };
    }

    pub fn len(&self) -> usize {
        return self.errors.len();
    }

    pub fn push(&mut self, error: T) {
        self.errors.push(error);
    }

    pub fn pop(&mut self) -> Option<T> {
        return self.errors.pop();
    }
}

impl<T: Error> From<Vec<T>> for Group<T> {
    fn from(value: Vec<T>) -> Self {
        return Self { errors: value };
    }
}

impl<T: Error + Clone> From<&[T]> for Group<T> {
    fn from(value: &[T]) -> Self {
        let mut errors: Vec<T> = Vec::new();

        for item in value {
            errors.push(item.clone());
        }

        return Self { errors };
    }
}

impl<T: Error> Display for Group<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for err in self.errors.iter() {
            Display::fmt(err, f)?;
        }

        return Ok(());
    }
}

impl<T: Error> Error for Group<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        for err in self.errors.iter() {
            err.source()?;
        }

        return None;
    }
}
