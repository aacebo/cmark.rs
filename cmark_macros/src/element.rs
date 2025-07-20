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
            children = input.parse::<Children>()?;
            input.parse::<tags::Close>()?;
        }

        return Ok(Self { open, children });
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let selector = &self.open.selector;
        let children = self.children.clone();
        let attributes = self.open.attributes.clone();
        let mut declaration = quote! {
            ::cmark::html::Element {
                selector: stringify!(#selector).to_string(),
                attributes: #attributes,
                children: #children,
            }.to_html()
        };

        if selector.to_token_stream().to_string() == ":: cmark :: html :: Fragment" {
            declaration = quote! {
                ::cmark::html::Fragment {
                    children: #children,
                }.to_html()
            };
        }

        declaration.to_tokens(tokens);
    }
}
