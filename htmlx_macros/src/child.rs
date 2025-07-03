use crate::element::Element;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream, Result};

#[derive(Clone)]
pub enum Child {
    Element(Element),
    RawBlock(syn::Block),
}

impl ToTokens for Child {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        return match self {
            Self::Element(element) => element.to_tokens(tokens),
            Self::RawBlock(block) => {
                let ts = if block.stmts.len() == 1 {
                    let first = &block.stmts[0];
                    quote!(#first)
                } else {
                    quote!(#block)
                };

                ts.to_tokens(tokens);
            }
        };
    }
}

impl Parse for Child {
    fn parse(input: ParseStream) -> Result<Self> {
        return match input.parse::<Element>() {
            Ok(element) => Ok(Self::Element(element)),
            Err(_) => {
                let block = input.parse::<syn::Block>()?;
                Ok(Self::RawBlock(block))
            }
        };
    }
}
