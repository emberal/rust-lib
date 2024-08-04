use proc_macro::TokenStream;

use quote::quote;
use syn::DeriveInput;

pub fn into_response_derive_impl(input: DeriveInput) -> TokenStream {
    let name = &input.ident;

    let expanded = quote! {
        impl IntoResponse for #name {
            fn into_response(self) -> axum::response::Response {
                let version = env!("CARGO_PKG_VERSION");
                lib::serde::response::BaseResponse::new(version, self)
                    .into_response()
            }
        }
    };

    TokenStream::from(expanded)
}
