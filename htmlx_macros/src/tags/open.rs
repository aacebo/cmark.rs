use syn::parse::Parse;

use crate::attributes::Map;

#[derive(Clone)]
pub struct Open {
    pub selector: syn::Path,
    pub attributes: Map,
    pub is_void: bool,
}

impl Parse for Open {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![<]>()?;
        let maybe_name = syn::Path::parse_mod_style(input);
        let name = maybe_name
            .unwrap_or_else(|_| syn::parse_str::<syn::Path>("::htmlx::Fragment").unwrap());
        let attributes = input.parse::<Map>()?;
        let is_void = input.parse::<syn::Token![/]>().is_ok();
        input.parse::<syn::Token![>]>()?;

        return Ok(Self {
            selector: name,
            attributes: attributes,
            is_void: is_void,
        });
    }
}
