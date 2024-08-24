extern crate proc_macro;

use crate::attributes::{extract_attrs, Attributes};
use crate::common::PrimaryKey;
use crate::create::derive_diesel_crud_create_impl;
use crate::delete::derive_diesel_crud_delete_impl;
use crate::list::derive_diesel_crud_list_impl;
use crate::read::derive_diesel_crud_read_impl;
use crate::update::derive_diesel_crud_update_impl;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod attributes;
mod common;
mod create;
mod delete;
mod list;
mod read;
mod update;

/// Derives 5 functions for CRUD operations
/// 1. create
/// 2. read
/// 3. update
/// 4. delete
/// 5. list
#[proc_macro_derive(DieselCrud, attributes(diesel_crud))]
pub fn derive_diesel_crud(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut item = parse_macro_input!(item as DeriveInput);
    let attrs = extract_attrs(&mut item).unwrap();

    let create = derive_diesel_crud_create_impl(&attrs);
    let read = derive_diesel_crud_read_impl(&attrs);
    let update = derive_diesel_crud_update_impl(&attrs);
    let delete = derive_diesel_crud_delete_impl(&attrs);
    let list = derive_diesel_crud_list_impl(&attrs);

    let Attributes {
        table,
        struct_ident,
        ..
    } = attrs;
    let expanded = quote! {
        #create
        #read
        #update
        #delete
        #list

        impl lib::diesel_crud_trait::DieselCrud<#table::table> for #struct_ident {}
    };
    expanded.into()
}

/// Derives the create function for CRUD operations.
/// Must be used on a struct.
/// # Struct Attributes
/// - table: Expr - The schema struct for the table (can be provided on either diesel or diesel_crud attribute)
/// - insert: Type - The insertable model (Optional, defaults to `Self`)
/// # Example
/// ```ignore
/// #[derive(Queryable, diesel_crud_derive::DieselCrudCreate)]
/// #[diesel_crud(create = crate::models::InsertUser)]
/// #[diesel(table_name = crate::schema::user)]
/// struct User {
///     #[diesel_crud(pk)]
///     email: String,
///     password: String,
/// }
/// ```
#[proc_macro_derive(DieselCrudCreate, attributes(diesel_crud))]
pub fn derive_diesel_crud_create(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut item = syn::parse_macro_input!(item as DeriveInput);
    let attrs = extract_attrs(&mut item).unwrap();
    derive_diesel_crud_create_impl(&attrs).into()
}

/// Derives the read function for CRUD operations.
/// Must be used on a struct with one field marked as the primary key.
/// # Struct Attributes
/// - table: Expr - The schema struct for the table (can be provided on either diesel or diesel_crud attribute)
/// # Field Attributes
/// - pk: Ident - The primary key field (Only one field should be marked as the primary key)
/// # Example
/// ```ignore
/// #[derive(Queryable, diesel_crud_derive::DieselCrudRead)]
/// #[diesel(table_name = crate::schema::user)]
/// struct User {
///     #[diesel_crud(pk)]
///     email: String,
///     password: String,
/// }
/// ```
#[proc_macro_derive(DieselCrudRead, attributes(diesel_crud))]
pub fn derive_diesel_crud_read(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut item = syn::parse_macro_input!(item as DeriveInput);
    let attrs = extract_attrs(&mut item).unwrap();
    derive_diesel_crud_read_impl(&attrs).into()
}

/// Derives the update function for CRUD operations.
/// Must be used on a struct.
/// # Struct Attributes
/// - table: Expr - The schema struct for the table (can be provided on either diesel or diesel_crud attribute)
/// - update: Type - The update model (Optional, defaults to `Self`)
/// # Example
/// ```ignore
/// #[derive(Queryable, diesel_crud_derive::DieselCrudUpdate)]
/// #[diesel(table_name = crate::schema::user)]
/// struct User {
///     #[diesel_crud(pk)]
///     email: String,
///     password: String,
/// }
/// ```
#[proc_macro_derive(DieselCrudUpdate, attributes(diesel_crud))]
pub fn derive_diesel_crud_update(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut item = syn::parse_macro_input!(item as DeriveInput);
    let attrs = extract_attrs(&mut item).unwrap();
    derive_diesel_crud_update_impl(&attrs).into()
}

/// Derives the delete function for CRUD operations.
/// Must be used on a struct with a field marked as primary key.
/// # Struct Attributes
/// - table: Expr - The schema struct for the table (can be provided on either diesel or diesel_crud attribute)
/// # Field Attributes
/// - pk: Ident - The primary key field (Only one field should be marked as the primary key)
/// # Example
/// ```ignore
/// #[derive(Queryable, diesel_crud_derive::DieselCrudDelete)]
/// #[diesel(table_name = crate::schema::user)]
/// struct User {
///     #[diesel_crud(pk)]
///     email: String,
///     password: String,
/// }
/// ```
#[proc_macro_derive(DieselCrudDelete, attributes(diesel_crud))]
pub fn derive_diesel_crud_delete(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut item = syn::parse_macro_input!(item as DeriveInput);
    let attrs = extract_attrs(&mut item).unwrap();
    derive_diesel_crud_delete_impl(&attrs).into()
}

/// Derives the list function for CRUD operations.
/// Must be used on a struct.
/// # Struct Attributes
/// - table: Expr - The schema struct for the table (can be provided on either diesel or diesel_crud attribute)
/// # Example
/// ```ignore
/// #[derive(Queryable, diesel_crud_derive::DieselCrudList)]
/// #[diesel(table_name = crate::schema::user)]
/// struct User {
///     #[diesel_crud(pk)]
///     email: String,
///     password: String,
/// }
/// ```
#[proc_macro_derive(DieselCrudList, attributes(diesel_crud))]
pub fn derive_diesel_crud_list(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut item = syn::parse_macro_input!(item as DeriveInput);
    let attrs = extract_attrs(&mut item).unwrap();
    derive_diesel_crud_list_impl(&attrs).into()
}
