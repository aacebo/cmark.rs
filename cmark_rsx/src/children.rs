#![allow(dead_code)]

use quote::{ToTokens, quote};
use syn::{
    Result,
    parse::{Parse, ParseStream},
};

use crate::child::Child;

#[derive(Clone)]
pub struct Children(Vec<Child>);

impl Children {
    pub fn new() -> Self {
        return Self(vec![]);
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
        return Self(value);
    }
}

impl Parse for Children {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut nodes = vec![];

        while !input.peek(syn::Token![<]) || !input.peek2(syn::Token![/]) {
            let child = input.parse::<Child>()?;
            nodes.push(child);
        }

        return Ok(Self::from(nodes));
    }
}

impl ToTokens for Children {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        return self.to_token_stream().to_tokens(tokens);
    }

    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        let children_quotes: Vec<_> = self.0.iter().map(|child| quote! { #child }).collect();

        match children_quotes.len() {
            0 => quote! { vec![] },
            _ => quote! { vec![#(#children_quotes),*] },
        }
    }
}
