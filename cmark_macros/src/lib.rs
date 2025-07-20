use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;

use crate::element::Element;

mod attributes;
mod child;
mod children;
mod element;
mod tags;

// #[proc_macro]
// #[proc_macro_error]
// pub fn html(input: TokenStream) -> TokenStream {
//     let el = proc_macro2::TokenStream::from(rsx(input));
//     let result = quote! { ::cmark::html::Element::render(#el, &::cmark::RenderOptions::default()) };
//     return TokenStream::from(result);
// }

#[proc_macro]
#[proc_macro_error]
pub fn rsx(input: TokenStream) -> TokenStream {
    let el = syn::parse_macro_input!(input as Element);
    let result = quote! { #el };
    return TokenStream::from(result);
}
