use crate::{common, StructAttributes};
use proc_macro2::Ident;
use quote::quote;
use syn::Expr;

pub(crate) fn derive_diesel_crud_create_impl(
    StructAttributes {
        table,
        entity,
        create,
        ..
    }: &StructAttributes,
    identifier: &Ident,
) -> proc_macro2::TokenStream {
    let body = function_body(table);
    let return_type = common::return_type(quote! { Self::Entity });

    quote! {
        #[automatically_derived]
        impl<'insertable, 'entity> lib::diesel_crud_trait::DieselCrudCreate<'insertable, 'entity, #table::table> for #identifier
            where
                'entity: 'insertable,
        {
            type Create = #create;
            type Entity = #entity;
            fn create<'a, 'b>(&'a self, create: &'insertable Self::Create) -> #return_type
                where
                    Self: Sync + 'a,
                    'a: 'b,
                    'insertable: 'b
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
        diesel_async::RunQueryDsl::get_result(
            diesel::dsl::insert_into(#table::table::table()).values(create),
            &mut connection
        )
            .await
            .map_err(Into::into)
    })
}
