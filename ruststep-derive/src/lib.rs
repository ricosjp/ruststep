//! Procedural macros for second step code generation
//!
//! ```text
//! ┌────────────────┐
//! │ EXPRESS schema │
//! └──┬─────────────┘
//!    │ esprc
//! ┌──▼─────────────────┐
//! │ Abstract Rust code │
//! └──┬─────────────────┘
//!    │ ruststep-derive
//! ┌──▼───────────────┐
//! │ Actual Rust code │
//! └──────────────────┘
//! ```
//!
//! Design
//! -------
//! - [espr::codegen::rust](../espr/codegen/rust/index.html)
//!   generates Rust code with proc-macros defined in this crate.
//! - This crate does not depends on espr explicitly.
//!

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::quote;
use std::convert::*;

mod common;
mod entity;
mod field_type;
mod holder_attr;
mod select;
mod table_init;
mod type_decl;

use common::*;
use field_type::*;
use holder_attr::*;
use table_init::*;

/// Derive `TableInit` for tables
///
/// ```
/// use ruststep_derive::{as_holder, Holder, TableInit};
/// use std::collections::HashMap;
///
/// #[derive(TableInit, Default)]
/// pub struct Table {
///     a: HashMap<u64, as_holder!(A)>,
///     b: HashMap<u64, as_holder!(B)>,
/// }
///
/// #[derive(Debug, Clone, PartialEq, Holder)]
/// #[holder(table = Table)]
/// #[holder(field = a)]
/// #[holder(generate_deserialize)]
/// pub struct A {
///     pub x: f64,
///     pub y: f64,
/// }
///
/// #[derive(Debug, Clone, PartialEq, Holder)]
/// #[holder(table = Table)]
/// #[holder(field = b)]
/// #[holder(generate_deserialize)]
/// pub struct B {
///     pub z: f64,
///     #[holder(use_place_holder)]
///     pub a: A,
/// }
/// ```
#[proc_macro_error]
#[proc_macro_derive(TableInit)]
pub fn derive_table_init_entry(input: TokenStream) -> TokenStream {
    derive_table_init(&syn::parse(input).unwrap()).into()
}

/// Generate `impl Deserialize` for entity structs
#[proc_macro_error]
#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize_entry(input: TokenStream) -> TokenStream {
    derive_deserialize(&syn::parse(input).unwrap()).into()
}

fn derive_deserialize(ast: &syn::DeriveInput) -> TokenStream2 {
    let ident = &ast.ident;
    match &ast.data {
        syn::Data::Struct(st) => entity::derive_deserialize(ident, st),
        syn::Data::Enum(e) => select::derive_deserialize(ident, e),
        _ => abort_call_site!("Only struct is supprted currently"),
    }
}

/// Generates `Holder` struct and related implementation for each `ENTITY` struct
///
/// `#[holder]` attribute
/// ---------------------
///
/// There are three types of attributes:
///
/// ```ignore
/// #[derive(Holder)]
/// #[holder(table = Table)] // <- container attribute
/// #[holder(field = b)]     // <- this is also a container attribute
/// pub struct B {
///     pub z: f64,
///     #[holder(use_place_holder)] // <- field attribute
///     pub a: A,
/// }
///
/// #[derive(Holder)]
/// #[holder(table = Table)] // <- container attribute
/// pub enum S2 {
///     #[holder(field = a)]        // <- variant attribute
///     #[holder(use_place_holder)] // <- this is also a variant attribute
///     A(Box<A>),
///     P(f64),
/// }
/// ```
///
/// - `#[holder(table = {path::to::table::struct})]`
///   - This must be a container attribute
///   - Specify a struct path which contains a table for this Holder
/// - `#[holder(field = {field_ident})]`
///   - This can be both in container or variant attribute
///   - Identifier of table field
/// - `#[holder(generate_deserialize)]`
///   - This must be a container attribute
///   - Flag for generating `impl Deserialize for XxxHolder`
/// - `#[holder(use_place_holder)]`
///   - This can be both in field or variant attribute
///   - Specify the field is not a simple type
///
#[proc_macro_error]
#[proc_macro_derive(Holder, attributes(holder))]
pub fn derive_holder_entry(input: TokenStream) -> TokenStream {
    derive_holder(&syn::parse(input).unwrap()).into()
}

fn derive_holder(ast: &syn::DeriveInput) -> TokenStream2 {
    let attr = HolderAttr::parse(&ast.attrs);
    let ident = &ast.ident;
    match &ast.data {
        syn::Data::Struct(st) => match st.fields {
            syn::Fields::Named(_) => entity::derive_holder(ident, st, &attr),
            syn::Fields::Unnamed(_) => type_decl::derive_holder(ident, st, &attr),
            syn::Fields::Unit => abort_call_site!("Unit struct is not supported."),
        },
        syn::Data::Enum(e) => select::derive_holder(ident, e, &attr),
        _ => abort_call_site!("Only struct is supprted currently"),
    }
}

/// Get `Holder` struct identifier from `ENTITY` struct identifier
///
/// - e.g. `as_holder!(A)` to `AHolder`
///
#[proc_macro_error]
#[proc_macro]
pub fn as_holder(input: TokenStream) -> TokenStream {
    let path = as_holder_path(&syn::parse(input).unwrap());
    let ts = quote! { #path };
    ts.into()
}
