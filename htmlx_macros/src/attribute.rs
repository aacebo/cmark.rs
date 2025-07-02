use common::collections::KeyValue;
use quote::quote;

use crate::to_stream::ToStream;

pub type Key = syn::punctuated::Punctuated<syn::Ident, syn::Token![-]>;
pub type Attribute = KeyValue<Key, Option<syn::Block>>;

trait _Attribute {
    fn identifiers(&self) -> Vec<&syn::Ident>;
}

impl _Attribute for Attribute {
    fn identifiers(&self) -> Vec<&syn::Ident> {
        return (&self.key).iter().collect::<Vec<_>>();
    }
}

impl ToStream for Attribute {
    fn to_stream(&self) -> proc_macro2::TokenStream {
        return match &self.value {
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
