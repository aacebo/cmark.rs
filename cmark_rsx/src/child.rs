use crate::element::Element;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream, Result};

#[derive(Clone)]
pub enum Child {
    Element(Element),
    RawBlock(syn::Block),
}

impl ToTokens for Child {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        return match self {
            Self::Element(element) => element.to_tokens(tokens),
            Self::RawBlock(block) => {
                let mut stream = TokenStream::new();

                stream.extend(
                    block
                        .stmts
                        .iter()
                        .map(|stmt| quote!(::cmark::html::Node::from(#stmt))),
                );

                stream.to_tokens(tokens);
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
