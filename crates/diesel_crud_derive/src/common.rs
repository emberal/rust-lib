use quote::quote;

pub(crate) fn function_body(body: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote! {
        let connection = self.pool.get().await;
        match connection {
            Ok(mut connection) => {
                use diesel::associations::HasTable;
                #body
            }
            Err(error) => Err(lib::diesel_crud_trait::CrudError::PoolError(error.to_string())),
        }
    }
}

pub(crate) fn return_type(output: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote! {
        std::pin::Pin<Box<dyn core::future::Future<Output = Result<#output, lib::diesel_crud_trait::CrudError>> + Send + 'b>>
    }
}
