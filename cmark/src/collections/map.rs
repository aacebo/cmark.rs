use super::key_value::KeyValue;

#[derive(Debug, Clone, Default)]
pub struct Map<K, V>
where
    K: PartialEq,
    K: Clone,
    V: Clone,
{
    items: Vec<KeyValue<K, V>>,
}

impl<K, V> Map<K, V>
where
    K: PartialEq,
    K: Clone,
    V: Clone,
{
    pub fn new() -> Self {
        return Self { items: Vec::new() };
    }

    pub fn at(&self, i: usize) -> V {
        return self.items[i].value.clone();
    }

    pub fn index(&self, key: K) -> Option<usize> {
        return self.items.iter().position(|pair| pair.key == key);
    }

    pub fn has(&self, key: K) -> bool {
        return self.items.iter().any(|pair| {
            return pair.key == key;
        });
    }

    pub fn get(&self, key: K) -> V {
        return match self.items.iter().find(|pair| pair.key == key) {
            Some(v) => v.value.clone(),
            None => panic!("map key not found"),
        };
    }

    pub fn try_get(&self, key: K) -> Option<V> {
        return match self.items.iter().find(|pair| pair.key == key) {
            Some(v) => Some(v.value.clone()),
            None => None,
        };
    }

    pub fn put(&mut self, key: K, value: V) {
        match self.index(key.clone()) {
            Some(i) => self.items[i].value = value,
            None => self.items.push(KeyValue::new(key, value)),
        };
    }

    pub fn del(&mut self, key: K) {
        match self.items.iter().position(|pair| pair.key == key) {
            Some(i) => self.items.swap_remove(i),
            None => panic!("map key not found"),
        };
    }
}
