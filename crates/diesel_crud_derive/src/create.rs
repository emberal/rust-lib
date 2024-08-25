use crate::{common, Attributes};
use quote::quote;

pub(crate) fn derive_diesel_crud_create_impl(
    Attributes {
        struct_ident,
        table,
        insert,
        ..
    }: &Attributes,
) -> proc_macro2::TokenStream {
    let return_type = common::return_type(quote! { Self });
    let many_return_type = common::return_type(quote! { Vec<Self> });

    quote! {
        #[automatically_derived]
        impl lib::diesel_crud_trait::DieselCrudCreate<#table::table> for #struct_ident {
            type Insert = #insert;
            fn insert<'a, 'async_trait>(insert: Self::Insert, conn: &'a mut diesel_async::AsyncPgConnection) -> #return_type
                where
                    Self: Sized + Sync + 'a,
                    'a: 'async_trait,
            {
                Box::pin(async move {
                    use diesel::associations::HasTable;
                    diesel_async::RunQueryDsl::get_result(
                        diesel::dsl::insert_into(#table::table::table()).values(insert),
                        conn
                    )
                        .await
                        .map_err(Into::into)
                })
            }

            fn insert_many<'a, 'b, 'async_trait>(insert: &'a [Self::Insert], conn: &'b mut diesel_async::AsyncPgConnection) -> #many_return_type
                where
                    Self: Sized + Sync + 'async_trait,
                    'a: 'async_trait,
                    'b: 'async_trait,
            {
                Box::pin(async move {
                    use diesel::associations::HasTable;
                    diesel_async::RunQueryDsl::get_results(
                        diesel::dsl::insert_into(#table::table::table()).values(insert),
                        conn
                    )
                        .await
                        .map_err(Into::into)
                })
            }
        }
    }
}
