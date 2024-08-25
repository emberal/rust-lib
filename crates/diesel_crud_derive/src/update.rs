use crate::{common, Attributes};
use quote::quote;

pub(crate) fn derive_diesel_crud_update_impl(
    Attributes {
        struct_ident,
        table,
        update,
        ..
    }: &Attributes,
) -> proc_macro2::TokenStream {
    let return_type = common::return_type(quote! { Self });

    quote! {
        #[automatically_derived]
        impl lib::diesel_crud_trait::DieselCrudUpdate for #struct_ident {
            type Update = #update;
            fn update<'a, 'async_trait>(update: Self::Update, conn: &'a mut diesel_async::AsyncPgConnection) -> #return_type
                where
                    Self: Sized + Sync + 'a,
                    'a: 'async_trait,
            {
                Box::pin(async move {
                    use diesel::associations::HasTable;
                    diesel_async::RunQueryDsl::get_result(
                        diesel::dsl::update(#table::table::table()).set(update),
                        conn,
                    )
                        .await
                        .map_err(Into::into)
                })
            }
        }
    }
}
