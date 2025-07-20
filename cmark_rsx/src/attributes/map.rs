#![allow(dead_code)]

use proc_macro_error::emit_error;
use quote::{ToTokens, quote};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

use super::attribute::{Attribute, Key};

#[derive(Clone, Default)]
pub struct Map {
    items: Vec<Attribute>,
}

impl Map {
    pub fn new() -> Self {
        return Self { items: Vec::new() };
    }

    pub fn at(&self, i: usize) -> Option<syn::Block> {
        return self.items[i].value.clone();
    }

    pub fn index(&self, key: Key) -> Option<usize> {
        return self
            .items
            .iter()
            .position(|pair| pair.identifiers() == key.iter().collect::<Vec<_>>());
    }

    pub fn has(&self, key: Key) -> bool {
        return self.items.iter().any(|pair| {
            return pair.identifiers() == key.iter().collect::<Vec<_>>();
        });
    }

    pub fn get(&self, key: Key) -> Option<syn::Block> {
        return match self
            .items
            .iter()
            .find(|pair| pair.identifiers() == key.iter().collect::<Vec<_>>())
        {
            Some(v) => v.value.clone(),
            None => panic!("map key not found"),
        };
    }

    pub fn try_get(&self, key: Key) -> Option<syn::Block> {
        return match self
            .items
            .iter()
            .find(|pair| pair.identifiers() == key.iter().collect::<Vec<_>>())
        {
            Some(v) => v.value.clone(),
            None => None,
        };
    }

    pub fn put(&mut self, key: Key, value: Option<syn::Block>) {
        match self.index(key.clone()) {
            Some(i) => self.items[i].value = value,
            None => self.items.push(Attribute::new(key, value)),
        };
    }

    pub fn put_object(&mut self, attribute: Attribute) {
        match self.index(attribute.clone().key) {
            Some(i) => self.items[i] = attribute,
            None => self.items.push(attribute),
        };
    }

    pub fn del(&mut self, key: Key) {
        match self
            .items
            .iter()
            .position(|pair| pair.identifiers() == key.iter().collect::<Vec<_>>())
        {
            Some(i) => self.items.swap_remove(i),
            None => panic!("map key not found"),
        };
    }

    pub fn drain(&mut self) -> std::vec::Drain<'_, Attribute> {
        return self.items.drain(0..);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Attribute> {
        return self.items.iter();
    }

    pub fn len(&self) -> usize {
        return self.items.len();
    }

    pub fn is_empty(&self) -> bool {
        return self.items.is_empty();
    }
}

impl Parse for Map {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attributes = Self::new();

        while input.peek(syn::Ident::peek_any) {
            let attribute = &input.parse::<Attribute>()?;
            let key = attribute.key.clone();

            if attributes.has(key.clone()) {
                emit_error!(
                    key.span(),
                    "There is a previous definition of the {} attribute",
                    quote!(#key)
                );
            }

            attributes.put_object(attribute.clone());
        }

        return Ok(attributes);
    }
}

impl ToTokens for Map {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.to_token_stream().to_tokens(tokens);
    }

    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        let attrs: Vec<_> = self
            .items
            .iter()
            .map(|attribute| {
                let mut iter = attribute.key.iter();
                let first_word = iter.next().unwrap().unraw();
                let ident = iter.fold(first_word.to_string(), |acc, curr| {
                    format!("{}-{}", acc, curr.unraw())
                });

                let value = attribute.to_token_stream();
                return quote! { attributes.put(#ident.to_string(), #value.to_string()); };
            })
            .collect();

        return quote! {{
            let mut attributes = cmark::html::Attributes::new();
            #(#attrs)*
            attributes
        }};
    }
}
