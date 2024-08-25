use crate::{common, Attributes};
use quote::quote;

pub(crate) fn derive_diesel_crud_list_impl(
    Attributes {
        struct_ident,
        table,
        ..
    }: &Attributes,
) -> proc_macro2::TokenStream {
    let return_type = common::return_type(quote! { Vec<Self> });

    quote! {
        #[automatically_derived]
        impl lib::diesel_crud_trait::DieselCrudList for #struct_ident {
            fn list<'a, 'async_trait>(conn: &'a mut diesel_async::AsyncPgConnection) -> #return_type
                where
                    Self: Sized + Sync + 'a,
                    'a: 'async_trait
            {
                Box::pin(async move {
                    use diesel::associations::HasTable;
                    diesel_async::RunQueryDsl::get_results(#table::table::table(), conn).await.map_err(Into::into)
                })
            }
        }
    }
}
