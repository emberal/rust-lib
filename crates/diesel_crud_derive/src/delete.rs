use crate::{common, Attributes, PrimaryKey};
use quote::quote;

pub(crate) fn derive_diesel_crud_delete_impl(
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
    let PrimaryKey {
        ident: pk_ident,
        ty: pk_type,
    } = pk.as_ref().unwrap();
    let return_type = common::return_type(quote! { Self });

    quote! {
        #[automatically_derived]
        impl lib::diesel_crud_trait::DieselCrudDelete for #struct_ident {
            type PK = #pk_type;
            fn delete<'a, 'b>(pk: Self::PK, conn: &'a mut diesel_async::AsyncPgConnection) -> #return_type
                where
                    Self: Sized + Sync + 'a,
                    'a: 'b,
            {
                Box::pin(async move {
                    use diesel::QueryDsl;
                    use diesel::associations::HasTable;
                    diesel_async::RunQueryDsl::get_result(
                        diesel::delete(
                            #table::table
                                .filter(diesel::expression_methods::ExpressionMethods::eq(#table::#pk_ident, pk))
                        ),
                        conn,
                    )
                        .await
                        .map_err(Into::into)
                })
            }
        }
    }
}
