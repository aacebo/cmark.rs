#![allow(dead_code)]

use syn::parse::Parse;

#[derive(Clone)]
pub struct Close {
    pub selector: syn::Path,
}

impl Parse for Close {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![<]>()?;
        input.parse::<syn::Token![/]>()?;
        let maybe_name = input.parse::<syn::Path>();
        input.parse::<syn::Token![>]>()?;
        let selector = maybe_name
            .unwrap_or_else(|_| syn::parse_str::<syn::Path>("::htmlx::Fragment").unwrap());

        return Ok(Self { selector });
    }
}
