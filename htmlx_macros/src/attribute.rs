use std::hash::{Hash, Hasher};

use common::{collections::KeyValue, validation::Validate};
use quote::quote;
use syn::ext::IdentExt;
use syn::parse::Parse;

use crate::to_stream::ToStream;

pub type Key = syn::punctuated::Punctuated<syn::Ident, syn::Token![-]>;

#[derive(Clone)]
pub struct Attribute(KeyValue<Key, Option<syn::Block>>);

impl Attribute {
    pub fn identifiers(&self) -> Vec<&syn::Ident> {
        return (&self.0.key).iter().collect::<Vec<_>>();
    }
}

impl From<KeyValue<Key, Option<syn::Block>>> for Attribute {
    fn from(value: KeyValue<Key, Option<syn::Block>>) -> Self {
        return Self { 0: value };
    }
}

impl ToStream for Attribute {
    fn to_stream(&self) -> proc_macro2::TokenStream {
        return match &self.0.value {
            None => quote!(#&self.key),
            Some(value) => {
                if value.stmts.len() == 1 {
                    let first = &value.stmts[0];
                    quote!(#first)
                } else {
                    quote!(#value)
                }
            }
        };
    }
}

impl Eq for Attribute {}

impl PartialEq for Attribute {
    fn eq(&self, other: &Self) -> bool {
        return self.identifiers() == other.identifiers();
    }
}

impl Hash for Attribute {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let identifiers = self.identifiers();
        return Hash::hash(&identifiers, state);
    }
}

impl Validate for Attribute {
    fn validate(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return None;
    }
}

impl Parse for Attribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = Key::parse_separated_nonempty_with(input, syn::Ident::parse_any)?;
        let has_value = input.peek(syn::Token![=]);

        if !has_value {
            return Ok(Self::from(KeyValue::new(name, None)));
        }

        input.parse::<syn::Token![=]>()?;
        let value = input.parse::<syn::Block>()?;
        return Ok(Self::from(KeyValue::new(name, Some(value))));
    }
}
