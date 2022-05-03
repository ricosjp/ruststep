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

#[cfg(test)]
mod snapshot_tests {
    use super::derive_holder;

    #[test]
    fn derive_holder_enum() {
        let input: syn::DeriveInput = syn::parse_str(
            r#"
            #[holder(table = Table)]
            #[holder(generate_deserialize)]
            pub enum S1 {
                #[holder(field = a)]
                #[holder(use_place_holder)]
                A(Box<A>),
                #[holder(field = b)]
                #[holder(use_place_holder)]
                B(Box<B>),
            }
            "#,
        )
        .unwrap();

        let tt = derive_holder(&input);
        let out = espr::codegen::rust::rustfmt(tt.to_string());

        insta::assert_snapshot!(out, @r###"
        #[derive(Clone, Debug, PartialEq)]
        pub enum S1Holder {
            A(Box<AHolder>),
            B(Box<BHolder>),
        }
        impl ::ruststep::tables::IntoOwned for S1Holder {
            type Owned = S1;
            type Table = Table;
            fn into_owned(self, table: &Self::Table) -> ::ruststep::error::Result<Self::Owned> {
                Ok(match self {
                    S1Holder::A(sub) => S1::A(Box::new(sub.into_owned(table)?)),
                    S1Holder::B(sub) => S1::B(Box::new(sub.into_owned(table)?)),
                })
            }
        }
        impl ::ruststep::tables::Holder for S1Holder {
            fn name() -> &'static str {
                "S1"
            }
            fn attr_len() -> usize {
                0
            }
        }
        impl<'de> ::ruststep::serde::de::Deserialize<'de> for S1Holder {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::ruststep::serde::de::Deserializer<'de>,
            {
                deserializer.deserialize_tuple_struct("S1", 0, S1HolderVisitor {})
            }
        }
        pub struct S1HolderVisitor;
        impl<'de> ::ruststep::serde::de::Visitor<'de> for S1HolderVisitor {
            type Value = S1Holder;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, "S1")
            }
            fn visit_map<A>(self, mut map: A) -> ::std::result::Result<Self::Value, A::Error>
            where
                A: ::ruststep::serde::de::MapAccess<'de>,
            {
                let key: String = map
                    .next_key()?
                    .expect("Empty map cannot be accepted as ruststep Holder");
                match key.as_str() {
                    "A" => {
                        let owned = map.next_value()?;
                        return Ok(S1Holder::A(Box::new(owned)));
                    }
                    "B" => {
                        let owned = map.next_value()?;
                        return Ok(S1Holder::B(Box::new(owned)));
                    }
                    _ => {
                        use ruststep::serde::de::{Error, Unexpected};
                        return Err(A::Error::invalid_value(Unexpected::Other(&key), &self));
                    }
                }
            }
        }
        impl ::ruststep::tables::WithVisitor for S1Holder {
            type Visitor = S1HolderVisitor;
            fn visitor_new() -> Self::Visitor {
                S1HolderVisitor {}
            }
        }
        impl ::ruststep::tables::EntityTable<S1Holder> for Table {
            fn get_owned(&self, entity_id: u64) -> ::ruststep::error::Result<S1> {
                if let Ok(owned) = ::ruststep::tables::get_owned(self, &self.a, entity_id) {
                    return Ok(S1::A(Box::new(owned.into())));
                }
                if let Ok(owned) = ::ruststep::tables::get_owned(self, &self.b, entity_id) {
                    return Ok(S1::B(Box::new(owned.into())));
                }
                Err(::ruststep::error::Error::UnknownEntity(entity_id))
            }
            fn owned_iter<'table>(
                &'table self,
            ) -> Box<dyn Iterator<Item = ::ruststep::error::Result<S1>> + 'table> {
                Box::new(::ruststep::itertools::chain![
                    ::ruststep::tables::owned_iter(self, &self.a)
                        .map(|owned| owned.map(|owned| S1::A(Box::new(owned.into())))),
                    ::ruststep::tables::owned_iter(self, &self.b)
                        .map(|owned| owned.map(|owned| S1::B(Box::new(owned.into()))))
                ])
            }
        }
        "###);
    }

    #[test]
    fn derive_holder_enum_any_subsuper() {
        let input: syn::DeriveInput = syn::parse_str(
            r#"
            # [holder (table = Tables)]
            #[holder(generate_deserialize)]
            pub enum BaseAny {
                #[holder(use_place_holder)]
                # [holder (field = base)]
                Base(Box<Base>),
                #[holder(use_place_holder)]
                # [holder (field = sub)]
                Sub(Box<SubAny>),
            }
            "#,
        )
        .unwrap();

        let tt = derive_holder(&input);
        let out = espr::codegen::rust::rustfmt(tt.to_string());

        insta::assert_snapshot!(out, @r###"
        #[derive(Clone, Debug, PartialEq)]
        pub enum BaseAnyHolder {
            Base(Box<BaseHolder>),
            Sub(Box<SubAnyHolder>),
        }
        impl ::ruststep::tables::IntoOwned for BaseAnyHolder {
            type Owned = BaseAny;
            type Table = Tables;
            fn into_owned(self, table: &Self::Table) -> ::ruststep::error::Result<Self::Owned> {
                Ok(match self {
                    BaseAnyHolder::Base(sub) => BaseAny::Base(Box::new(sub.into_owned(table)?)),
                    BaseAnyHolder::Sub(sub) => BaseAny::Sub(Box::new(sub.into_owned(table)?)),
                })
            }
        }
        impl ::ruststep::tables::Holder for BaseAnyHolder {
            fn name() -> &'static str {
                "BASE_ANY"
            }
            fn attr_len() -> usize {
                0
            }
        }
        impl<'de> ::ruststep::serde::de::Deserialize<'de> for BaseAnyHolder {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::ruststep::serde::de::Deserializer<'de>,
            {
                deserializer.deserialize_tuple_struct("BASE_ANY", 0, BaseAnyHolderVisitor {})
            }
        }
        pub struct BaseAnyHolderVisitor;
        impl<'de> ::ruststep::serde::de::Visitor<'de> for BaseAnyHolderVisitor {
            type Value = BaseAnyHolder;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, "BASE_ANY")
            }
            fn visit_map<A>(self, mut map: A) -> ::std::result::Result<Self::Value, A::Error>
            where
                A: ::ruststep::serde::de::MapAccess<'de>,
            {
                let key: String = map
                    .next_key()?
                    .expect("Empty map cannot be accepted as ruststep Holder");
                match key.as_str() {
                    "BASE" => {
                        let owned = map.next_value()?;
                        return Ok(BaseAnyHolder::Base(Box::new(owned)));
                    }
                    "SUB" => {
                        let owned = map.next_value()?;
                        return Ok(BaseAnyHolder::Sub(Box::new(owned)));
                    }
                    _ => {
                        use ruststep::serde::de::{Error, Unexpected};
                        return Err(A::Error::invalid_value(Unexpected::Other(&key), &self));
                    }
                }
            }
        }
        impl ::ruststep::tables::WithVisitor for BaseAnyHolder {
            type Visitor = BaseAnyHolderVisitor;
            fn visitor_new() -> Self::Visitor {
                BaseAnyHolderVisitor {}
            }
        }
        impl ::ruststep::tables::EntityTable<BaseAnyHolder> for Tables {
            fn get_owned(&self, entity_id: u64) -> ::ruststep::error::Result<BaseAny> {
                if let Ok(owned) = ::ruststep::tables::get_owned(self, &self.base, entity_id) {
                    return Ok(BaseAny::Base(Box::new(owned.into())));
                }
                if let Ok(owned) = ::ruststep::tables::get_owned(self, &self.sub, entity_id) {
                    return Ok(BaseAny::Sub(Box::new(owned.into())));
                }
                Err(::ruststep::error::Error::UnknownEntity(entity_id))
            }
            fn owned_iter<'table>(
                &'table self,
            ) -> Box<dyn Iterator<Item = ::ruststep::error::Result<BaseAny>> + 'table> {
                Box::new(::ruststep::itertools::chain![
                    ::ruststep::tables::owned_iter(self, &self.base)
                        .map(|owned| owned.map(|owned| BaseAny::Base(Box::new(owned.into())))),
                    ::ruststep::tables::owned_iter(self, &self.sub)
                        .map(|owned| owned.map(|owned| BaseAny::Sub(Box::new(owned.into()))))
                ])
            }
        }
        "###);
    }

    #[test]
    fn skip_unrelated_attributes() {
        let input: syn::DeriveInput = syn::parse_str(
            r#"
            #[derive(
                Debug, Clone, PartialEq, AsRef, AsMut, Deref, DerefMut, :: derive_new :: new, Holder,
            )]
            # [holder (table = Tables)]
            # [holder (field = sub1)]
            #[holder(generate_deserialize)]
            pub struct Sub1 {
                #[as_ref]
                #[as_mut]
                #[deref]
                #[deref_mut]
                #[holder(use_place_holder)]
                pub base: Base,
                pub y1: f64,
            }
            "#,
        )
        .unwrap();

        let tt = derive_holder(&input);
        let out = espr::codegen::rust::rustfmt(tt.to_string());

        insta::assert_snapshot!(out, @r###"
        #[doc = r" Auto-generated by `#[derive(Holder)]`"]
        #[derive(Debug, Clone, PartialEq)]
        pub struct Sub1Holder {
            pub base: ::ruststep::tables::PlaceHolder<BaseHolder>,
            pub y1: f64,
        }
        #[automatically_derived]
        impl ::ruststep::tables::IntoOwned for Sub1Holder {
            type Table = Tables;
            type Owned = Sub1;
            fn into_owned(self, table: &Self::Table) -> ::ruststep::error::Result<Self::Owned> {
                let Sub1Holder { base, y1 } = self;
                Ok(Sub1 {
                    base: base.into_owned(table)?,
                    y1: y1,
                })
            }
        }
        #[automatically_derived]
        impl ::ruststep::tables::Holder for Sub1Holder {
            fn name() -> &'static str {
                "SUB_1"
            }
            fn attr_len() -> usize {
                2usize
            }
        }
        #[automatically_derived]
        impl ::ruststep::tables::EntityTable<Sub1Holder> for Tables {
            fn get_owned(&self, entity_id: u64) -> ::ruststep::error::Result<Sub1> {
                ::ruststep::tables::get_owned(self, &self.sub1, entity_id)
            }
            fn owned_iter<'table>(
                &'table self,
            ) -> Box<dyn Iterator<Item = ::ruststep::error::Result<Sub1>> + 'table> {
                ::ruststep::tables::owned_iter(self, &self.sub1)
            }
        }
        #[doc(hidden)]
        pub struct Sub1HolderVisitor;
        #[automatically_derived]
        impl<'de> ::ruststep::serde::de::Visitor<'de> for Sub1HolderVisitor {
            type Value = Sub1Holder;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, "SUB_1")
            }
            fn visit_seq<A>(self, mut seq: A) -> ::std::result::Result<Self::Value, A::Error>
            where
                A: ::ruststep::serde::de::SeqAccess<'de>,
            {
                if let Some(size) = seq.size_hint() {
                    if size != 2usize {
                        use ruststep::serde::de::Error;
                        return Err(A::Error::invalid_length(size, &self));
                    }
                }
                let base = seq.next_element()?.unwrap();
                let y1 = seq.next_element()?.unwrap();
                Ok(Sub1Holder { base, y1 })
            }
            fn visit_map<A>(self, mut map: A) -> ::std::result::Result<Self::Value, A::Error>
            where
                A: ::ruststep::serde::de::MapAccess<'de>,
            {
                let key: String = map
                    .next_key()?
                    .expect("Empty map cannot be accepted as ruststep Holder");
                if key != "SUB_1" {
                    use ruststep::serde::de::{Error, Unexpected};
                    return Err(A::Error::invalid_value(Unexpected::Other(&key), &self));
                }
                let value = map.next_value()?;
                Ok(value)
            }
        }
        #[automatically_derived]
        impl<'de> ::ruststep::serde::de::Deserialize<'de> for Sub1Holder {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::ruststep::serde::de::Deserializer<'de>,
            {
                deserializer.deserialize_tuple_struct("SUB_1", 2usize, Sub1HolderVisitor {})
            }
        }
        #[automatically_derived]
        impl ::ruststep::tables::WithVisitor for Sub1Holder {
            type Visitor = Sub1HolderVisitor;
            fn visitor_new() -> Self::Visitor {
                Sub1HolderVisitor {}
            }
        }
        "###);
    }
}
