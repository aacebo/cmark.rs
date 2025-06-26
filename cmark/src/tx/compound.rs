#[derive(Debug, Clone)]
pub struct Compound<T>
where
    T: Clone,
{
    items: Vec<super::Tx<T>>,
}

impl<T> Compound<T>
where
    T: Clone,
{
    pub fn new(items: &[super::Tx<T>]) -> Self {
        return Self {
            items: items.to_vec(),
        };
    }

    pub fn rollback(&self) {
        for item in &self.items {
            item.rollback();
        }
    }
}
