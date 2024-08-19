use crate::{common, StructAttributes};
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::Expr;

pub(crate) fn derive_diesel_crud_delete_impl(
    StructAttributes {
        table,
        pk,
        pk_field,
        ..
    }: &StructAttributes,
    identifier: &Ident,
) -> proc_macro2::TokenStream {
    let body = function_body(
        table,
        pk_field
            .clone()
            .map(Expr::into_token_stream)
            .unwrap_or_else(|| quote! { id }),
    );
    let return_type = common::return_type(quote! { usize });

    quote! {
        #[automatically_derived]
        impl lib::diesel_crud_trait::DieselCrudDelete for #identifier {
            type PK = #pk;
            fn delete<'a, 'pk, 'b>(&'a self, pk: &'pk Self::PK) -> #return_type
                where
                    Self: Sync + 'a,
                    'a: 'b,
                    'pk: 'b,
            {
                Box::pin(async move {
                    #body
                })
            }
        }
    }
}

fn function_body(table: &Expr, pk_field: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    common::function_body(quote! {
        use diesel::QueryDsl;
        diesel_async::RunQueryDsl::execute(
            diesel::delete(
                #table::table
                    .filter(diesel::expression_methods::ExpressionMethods::eq(#table::#pk_field, pk))
            ),
            &mut connection,
        )
            .await
            .map_err(Into::into)
    })
}
