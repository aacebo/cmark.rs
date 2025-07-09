use std::collections::{BTreeSet, btree_set::Iter};

#[derive(Debug, Clone)]
pub struct Classes(BTreeSet<String>);

impl Classes {
    pub fn new() -> Self {
        return Self { 0: BTreeSet::new() };
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn contains(&self, name: &str) -> bool {
        return self.0.contains(name);
    }

    pub fn add(&mut self, name: &str) -> bool {
        return self.0.insert(name.to_string());
    }

    pub fn remove(&mut self, name: &str) -> bool {
        return self.0.remove(name);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn iter(&self) -> Iter<'_, String> {
        return self.0.iter();
    }
}

impl From<BTreeSet<String>> for Classes {
    fn from(value: BTreeSet<String>) -> Self {
        return Self { 0: value };
    }
}

impl From<String> for Classes {
    fn from(value: String) -> Self {
        let mut set: BTreeSet<String> = BTreeSet::new();

        for class in value.split(' ') {
            set.insert(class.to_string());
        }

        return Self::from(set);
    }
}

impl ToString for Classes {
    fn to_string(&self) -> String {
        let mut value: Vec<String> = vec![];

        for class in self.iter() {
            value.push(class.clone());
        }

        return value.join(" ");
    }
}
