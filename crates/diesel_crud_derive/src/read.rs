use crate::common::PrimaryKey;
use crate::{common, Attributes};
use quote::quote;

pub(crate) fn derive_diesel_crud_read_impl(
    Attributes {
        struct_ident,
        table,
        pk,
        ..
    }: &Attributes,
) -> proc_macro2::TokenStream {
    if pk.is_none() {
        panic!("Please specify a primary key using #[diesel_crud(pk)]");
    }
    let PrimaryKey { ty: pk_type, .. } = pk.as_ref().unwrap();
    let return_type = common::return_type(quote! { Self });

    quote! {
        #[automatically_derived]
        impl lib::diesel_crud_trait::DieselCrudRead for #struct_ident {
            type PK = #pk_type;
            fn read<'a, 'async_trait>(pk: Self::PK, conn: &'a mut diesel_async::AsyncPgConnection) -> #return_type
                where
                    Self: Sized + Sync + 'a,
                    'a: 'async_trait
            {
                Box::pin(async move {
                    use diesel::associations::HasTable;
                    diesel_async::RunQueryDsl::get_result(
                        diesel::QueryDsl::find(#table::table::table(), pk),
                        conn
                    )
                        .await
                        .map_err(Into::into)
                })
            }
        }
    }
}
