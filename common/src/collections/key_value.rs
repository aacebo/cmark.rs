#[derive(Debug, Clone)]
pub struct KeyValue<K, V> {
    pub key: K,
    pub value: V,
}

impl<K, V> KeyValue<K, V> {
    pub fn new(key: K, value: V) -> Self {
        return Self { key, value };
    }
}
