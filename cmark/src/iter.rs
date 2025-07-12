use crate::ParseError;

pub trait Iter<K: PartialEq, V> {
    fn next(&mut self) -> Option<V>;
    fn next_if(&mut self, value: K) -> Option<V>;
    fn next_or_err(&mut self, value: K) -> Result<V, ParseError>;
    fn next_while(&mut self, value: K) -> Vec<V>;
    fn next_until(&mut self, value: K) -> Vec<V>;
    fn next_n(&mut self, value: K, n: i32) -> Vec<V>;
}
