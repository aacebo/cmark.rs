#![allow(dead_code)]

use quote::ToTokens;
use syn::{
    Result,
    parse::{Parse, ParseStream},
};

use crate::child::Child;

#[derive(Clone)]
pub struct Children(Vec<Child>);

impl Children {
    pub fn new() -> Self {
        return Self { 0: vec![] };
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn iter(&self) -> core::slice::Iter<Child> {
        return self.0.iter();
    }

    pub fn push(&mut self, child: Child) {
        self.0.push(child);
    }

    pub fn pop(&mut self) -> Option<Child> {
        return self.0.pop();
    }
}

impl From<Vec<Child>> for Children {
    fn from(value: Vec<Child>) -> Self {
        return Self { 0: value };
    }
}

impl Parse for Children {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut nodes = vec![];

        while !input.peek(syn::Token![<]) || !input.peek2(syn::Token![/]) {
            let child = input.parse::<Child>()?;
            nodes.push(child);
        }

        Ok(Self::from(nodes))
    }
}

impl ToTokens for Children {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for child in self.0.iter() {
            child.to_tokens(tokens);
        }
    }
}
