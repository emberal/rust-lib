use crate::common::PrimaryKey;
use deluxe::{extract_attributes, ExtractAttributes};
use proc_macro2::Ident;
use quote::quote;
use std::collections::HashMap;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Expr, Path, Type};

#[derive(ExtractAttributes)]
#[deluxe(attributes(diesel))]
pub(crate) struct DieselStructAttributes {
    table_name: Option<Expr>,
    #[deluxe(rest)]
    _rest: HashMap<Path, Expr>,
}

#[derive(ExtractAttributes)]
#[deluxe(attributes(diesel_crud))]
pub(crate) struct StructAttributes {
    table: Option<Expr>,
    #[deluxe(default)]
    insert: Option<Type>,
    #[deluxe(default)]
    update: Option<Type>,
}

#[derive(ExtractAttributes)]
#[deluxe(attributes(diesel_crud))]
pub(crate) struct FieldAttributes(#[allow(unused)] Expr);

pub(crate) struct Attributes {
    pub struct_ident: Ident,
    pub table: Expr,
    pub insert: Type,
    pub update: Type,
    pub pk: Option<PrimaryKey>,
}

pub(crate) fn extract_attrs(ast: &mut DeriveInput) -> deluxe::Result<Attributes> {
    let struct_attributes: StructAttributes = extract_attributes(ast)?;
    let diesel_attributes: DieselStructAttributes = extract_attributes(ast)?;
    Ok(Attributes {
        struct_ident: ast.ident.clone(),
        table: diesel_attributes.table_name.unwrap_or_else(|| {
            struct_attributes
                .table
                .expect("Table name should be provided on either diesel or diesel_crud attribute")
        }),
        insert: struct_attributes
            .insert
            .unwrap_or_else(|| Type::Verbatim(quote! { Self })),
        update: struct_attributes
            .update
            .unwrap_or_else(|| Type::Verbatim(quote! { Self })),
        pk: extract_field_attrs(ast).ok(),
    })
}

fn extract_field_attrs(ast: &mut DeriveInput) -> deluxe::Result<PrimaryKey> {
    if let Data::Struct(data_struct) = &mut ast.data {
        for field in data_struct.fields.iter_mut() {
            if let Ok(FieldAttributes(_)) = extract_attributes(field) {
                return Ok(PrimaryKey {
                    ident: field.ident.clone().unwrap(),
                    ty: field.ty.clone(),
                });
            }
        }
    } else {
        return Err(deluxe::Error::new(ast.span(), "Expected a struct"));
    };
    Err(deluxe::Error::new(ast.span(), "Primary key not found"))
}
