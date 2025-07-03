use quote::{ToTokens, quote};
use syn::parse::Parse;

use crate::{children::Children, tags};

#[derive(Clone)]
pub struct Element {
    pub open: tags::Open,
    pub children: Children,
}

impl Parse for Element {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let open = input.parse::<tags::Open>()?;
        let mut children = Children::new();

        if !open.is_void {
            children = Children::parse(input)?;
            input.parse::<tags::Close>()?;
        }

        return Ok(Self { open, children });
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let selector = &self.open.selector;
        let children = &self.children;
        let attributes = self.open.attributes.clone();
        let declaration = quote! {
            ::htmlx::HTMLElement {
                selector: stringify!(#selector),
                attributes: #attributes,
                contents: #children,
            }
        };

        declaration.to_tokens(tokens);
    }
}
