use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Tx<T> {
    copy: T,
    ptr: Rc<RefCell<T>>,
}

impl<T> Tx<T>
where
    T: Clone,
{
    pub fn new(data: Rc<RefCell<T>>) -> Self {
        return Self {
            copy: data.borrow().clone(),
            ptr: data.clone(),
        };
    }

    pub fn rollback(&self) {
        self.ptr.replace(self.copy.clone());
    }
}
