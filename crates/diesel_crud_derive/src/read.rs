use proc_macro2::Ident;
use quote::quote;
use syn::Expr;

use crate::{common, StructAttributes};

pub(crate) fn derive_diesel_crud_read_impl(
    StructAttributes {
        table, entity, pk, ..
    }: &StructAttributes,
    identifier: &Ident,
) -> proc_macro2::TokenStream {
    let body = function_body(table);
    let return_type = common::return_type(quote! { Self::Entity });

    quote! {
        #[automatically_derived]
        impl lib::diesel_crud_trait::DieselCrudRead for #identifier {
            type PK = #pk;
            type Entity = #entity;
            fn read<'a, 'b>(&'a self, pk: Self::PK) -> #return_type
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
        diesel_async::RunQueryDsl::get_result(diesel::QueryDsl::find(#table::table::table(), pk), &mut connection).await.map_err(Into::into)
    })
}
