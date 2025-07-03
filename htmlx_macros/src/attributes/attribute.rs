use std::hash::{Hash, Hasher};

use quote::{ToTokens, quote};
use syn::ext::IdentExt;
use syn::parse::Parse;

pub type Key = syn::punctuated::Punctuated<syn::Ident, syn::Token![-]>;

#[derive(Clone)]
pub struct Attribute {
    pub key: Key,
    pub value: Option<syn::Block>,
}

impl Attribute {
    pub fn new(key: Key, value: Option<syn::Block>) -> Self {
        return Self { key, value };
    }

    pub fn identifiers(&self) -> Vec<&syn::Ident> {
        return (&self.key).iter().collect::<Vec<_>>();
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let stream = self.to_token_stream();
        return stream.to_tokens(tokens);
    }

    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        let key = &self.key;
        return match &self.value {
            None => quote!(#key),
            Some(value) => {
                if value.stmts.len() == 1 {
                    let first = &value.stmts[0];
                    quote!({ #key: #first })
                } else {
                    quote!({ #key: #value })
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

impl Parse for Attribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = Key::parse_separated_nonempty_with(input, syn::Ident::parse_any)?;
        let has_value = input.peek(syn::Token![=]);

        if !has_value {
            return Ok(Self::new(name, None));
        }

        input.parse::<syn::Token![=]>()?;
        let value = input.parse::<syn::Block>()?;
        return Ok(Self::new(name, Some(value)));
    }
}
