use crate::{common, StructAttributes};
use proc_macro2::Ident;
use quote::quote;
use syn::Expr;

pub(crate) fn derive_diesel_crud_update_impl(
    StructAttributes { table, update, .. }: &StructAttributes,
    identifier: &Ident,
) -> proc_macro2::TokenStream {
    let body = function_body(table);
    let return_type = common::return_type(quote! { usize });

    quote! {
        #[automatically_derived]
        impl lib::diesel_crud_trait::DieselCrudUpdate for #identifier {
            type Update = #update;
            fn update<'a, 'b>(&'a self, update: Self::Update) -> #return_type
                where
                    Self: Sync + 'a,
                    'a: 'b,
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
        diesel_async::RunQueryDsl::execute(
            diesel::dsl::update(#table::table::table()).set(update),
            &mut connection,
        )
            .await
            .map_err(Into::into)
    })
}
