use proc_macro2::Ident;
use quote::quote;
use syn::Type;

pub(crate) struct PrimaryKey {
    pub ident: Ident,
    pub ty: Type,
}

pub(crate) fn return_type(output: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote! {
        std::pin::Pin<Box<dyn core::future::Future<Output = Result<#output, lib::diesel_crud_trait::CrudError>> + Send + 'b>>
    }
}
