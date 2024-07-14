extern crate proc_macro;
use {
    proc_macro::TokenStream,
    syn::{parse_macro_input, DeriveInput},
};

mod derive;

#[proc_macro_derive(IntoResponse)]
pub fn into_response_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive::into_response_derive_impl(input)
}
