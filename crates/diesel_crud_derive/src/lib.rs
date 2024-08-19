extern crate proc_macro;

use crate::create::derive_diesel_crud_create_impl;
use crate::delete::derive_diesel_crud_delete_impl;
use crate::list::derive_diesel_crud_list_impl;
use crate::read::derive_diesel_crud_read_impl;
use crate::update::derive_diesel_crud_update_impl;
use deluxe::{extract_attributes, ExtractAttributes};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Expr, Type};

mod common;
mod create;
mod delete;
mod list;
mod read;
mod update;

#[derive(ExtractAttributes)]
#[deluxe(attributes(diesel_crud))]
pub(crate) struct StructAttributes {
    table: Expr,
    #[deluxe(default)]
    entity: Option<Type>,
    #[deluxe(default)]
    pk: Option<Type>,
    #[deluxe(default)]
    pk_field: Option<Expr>,
    #[deluxe(default)]
    create: Option<Type>, // TODO if None, use entity?
    #[deluxe(default)]
    update: Option<Type>, // TODO if None, use entity?
}

// TODO get pool field automatically or by attribute

/// Derives 5 functions for CRUD operations
/// 1. create
/// 2. read
/// 3. update
/// 4. delete
/// 5. list
#[proc_macro_derive(DieselCrud, attributes(diesel_crud))]
pub fn derive_diesel_crud(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut item = parse_macro_input!(item as DeriveInput);
    let struct_attributes = extract_attributes(&mut item).unwrap();
    let identifier = item.ident;

    let create = derive_diesel_crud_create_impl(&struct_attributes, &identifier);
    let read = derive_diesel_crud_read_impl(&struct_attributes, &identifier);
    let update = derive_diesel_crud_update_impl(&struct_attributes, &identifier);
    let delete = derive_diesel_crud_delete_impl(&struct_attributes, &identifier);
    let list = derive_diesel_crud_list_impl(&struct_attributes, &identifier);

    let table = struct_attributes.table;
    let expanded = quote! {
        #create
        #read
        #update
        #delete
        #list

        impl<'insertable, 'entity> lib::diesel_crud_trait::DieselCrud<'insertable, 'entity, #table::table> for #identifier
            where
                'entity: 'insertable
        {}
    };
    expanded.into()
}

/// Derives the create function for CRUD operations.
/// Must be used on a struct with a field named `pool`, containing a `Pool<AsyncPgConnection>`.
/// # Struct Attributes
/// - table: Ident - The schema struct for the table
/// - result: Type - The resulting model
/// - create: Type - The insertable model
/// # Example
/// ```ignore
/// use diesel_async::{AsyncPgConnection, pooled_connection::deadpool::Pool};
///
/// #[derive(diesel_crud_derive::DieselCrudCreate)]
/// #[diesel_crud(table = crate::schema::user, result = crate::models::User, create = crate::models::InsertUser)]
/// struct TestServiceCreate {
///    pool: Pool<AsyncPgConnection>,
/// }
/// ```
#[proc_macro_derive(DieselCrudCreate, attributes(diesel_crud))]
pub fn derive_diesel_crud_create(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut item = syn::parse_macro_input!(item as DeriveInput);
    let struct_attributes = extract_attributes(&mut item).unwrap();
    derive_diesel_crud_create_impl(&struct_attributes, &item.ident).into()
}

/// Derives the read function for CRUD operations.
/// Must be used on a struct with a field named `pool`, containing a `Pool<AsyncPgConnection>`.
/// # Struct Attributes
/// - table: Ident - The schema struct for the table
/// - pk: Type - The primary key type
/// - result: Type - The resulting model
/// - pk_field (optional): Expr - The field to use as the primary key. Defaults to `id`
/// # Example
/// ```ignore
/// use diesel_async::{AsyncPgConnection, pooled_connection::deadpool::Pool};
///
/// #[derive(diesel_crud_derive::DieselCrudRead)]
/// #[diesel_crud(table = crate::schema::user, result = crate::models::User, pk = String)]
/// struct TestServiceRead {
///    pool: Pool<AsyncPgConnection>,
/// }
/// ```
#[proc_macro_derive(DieselCrudRead, attributes(diesel_crud))]
pub fn derive_diesel_crud_read(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut item = syn::parse_macro_input!(item as DeriveInput);
    let struct_attributes = extract_attributes(&mut item).unwrap();
    derive_diesel_crud_read_impl(&struct_attributes, &item.ident).into()
}

/// Derives the update function for CRUD operations.
/// Must be used on a struct with a field named `pool`, containing a `Pool<AsyncPgConnection>`.
/// # Struct Attributes
/// - table: Ident - The schema struct for the table
/// - update: Type - The update model
/// # Example
/// ```ignore
/// use diesel_async::{AsyncPgConnection, pooled_connection::deadpool::Pool};
///
/// #[derive(diesel_crud_derive::DieselCrudUpdate)]
/// #[diesel_crud(table = crate::schema::user, update = crate::models::User)]
/// struct TestServiceUpdate {
///   pool: Pool<AsyncPgConnection>,
/// }
/// ```
#[proc_macro_derive(DieselCrudUpdate, attributes(diesel_crud))]
pub fn derive_diesel_crud_update(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut item = syn::parse_macro_input!(item as DeriveInput);
    let struct_attributes = extract_attributes(&mut item).unwrap();
    derive_diesel_crud_update_impl(&struct_attributes, &item.ident).into()
}

/// Derives the delete function for CRUD operations.
/// Must be used on a struct with a field named `pool`, containing a `Pool<AsyncPgConnection>`.
/// # Struct Attributes
/// - table: Ident - The schema struct for the table
/// - pk: Type - The primary key type
/// - pk_field (optional): Expr - The field to use as the primary key. Defaults to `id`
/// # Example
/// ```ignore
/// use diesel_async::{AsyncPgConnection, pooled_connection::deadpool::Pool};
///
/// #[derive(diesel_crud_derive::DieselCrudDelete)]
/// #[diesel_crud(table = crate::schema::user, pk = String)]
/// struct TestServiceDelete {
///  pool: Pool<AsyncPgConnection>,
/// }
/// ```
#[proc_macro_derive(DieselCrudDelete, attributes(diesel_crud))]
pub fn derive_diesel_crud_delete(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut item = syn::parse_macro_input!(item as DeriveInput);
    let struct_attributes = extract_attributes(&mut item).unwrap();
    derive_diesel_crud_delete_impl(&struct_attributes, &item.ident).into()
}

/// Derives the list function for CRUD operations.
/// Must be used on a struct with a field named `pool`, containing a `Pool<AsyncPgConnection>`.
/// # Struct Attributes
/// - table: Ident - The schema struct for the table
/// - result: Type - The resulting model
/// # Example
/// ```ignore
/// use diesel_async::{AsyncPgConnection, pooled_connection::deadpool::Pool};
///
/// #[derive(diesel_crud_derive::DieselCrudList)]
/// #[diesel_crud(table = crate::schema::user, result = crate::models::User)]
/// struct TestServiceList {
///   pool: Pool<AsyncPgConnection>,
/// }
/// ```
#[proc_macro_derive(DieselCrudList, attributes(diesel_crud))]
pub fn derive_diesel_crud_list(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut item = syn::parse_macro_input!(item as DeriveInput);
    let struct_attributes = extract_attributes(&mut item).unwrap();
    derive_diesel_crud_list_impl(&struct_attributes, &item.ident).into()
}
