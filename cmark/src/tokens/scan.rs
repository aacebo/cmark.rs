use crate::tokens::Token;

pub trait Scan {
    fn scan(&self) -> Option<Token>;
}
