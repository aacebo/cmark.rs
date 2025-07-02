pub trait ToStream {
    fn to_stream(&self) -> proc_macro2::TokenStream;
}
