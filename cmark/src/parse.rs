use crate::{
    ParseError, html,
    tokens::{Iter, Scan},
};

pub trait Parse {
    fn parse<Scanner: Scan>(&self, iter: &mut Iter<Scanner>) -> Result<html::Node, ParseError>;
}
