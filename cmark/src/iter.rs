use crate::ParseError;

pub trait Iter<K: PartialEq, V>: Iterator {
    fn next_if(&mut self, key: K) -> Option<V>;
    fn next_or_err(&mut self, key: K, err: &'_ str) -> Result<V, ParseError>;
    fn next_while(&mut self, key: K) -> i32;
    fn next_until(&mut self, key: K) -> i32;
    fn next_n(&mut self, key: K, n: i32) -> i32;
}
