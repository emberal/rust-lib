use proc_macro2::Ident;
use quote::quote;
use syn::Expr;

use crate::{common, StructAttributes};

pub(crate) fn derive_diesel_crud_list_impl(
    StructAttributes { table, entity, .. }: &StructAttributes,
    identifier: &Ident,
) -> proc_macro2::TokenStream {
    let body = function_body(table);
    let return_type = common::return_type(quote! { Vec<Self::Entity> });

    quote! {
        #[automatically_derived]
        impl lib::diesel_crud_trait::DieselCrudList for #identifier {
            type Entity = #entity;
            fn list<'a, 'b>(&'a self) -> #return_type
                where
                    Self: Sync + 'a,
                    'a: 'b
            {
                Box::pin(async move {
                    #body
                })
            }
        }
    }
}

fn function_body(table: &Expr) -> proc_macro2::TokenStream {
    common::function_body(quote! {
        diesel_async::RunQueryDsl::get_results(#table::table::table(), &mut connection).await.map_err(Into::into)
    })
}
