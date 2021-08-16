//! Procedural macros for ruststep
//!
//! ```
//! use ruststep_derive::{as_holder, Holder};
//! use std::collections::HashMap;
//!
//! pub struct Table {
//!     a: HashMap<u64, as_holder!(A)>,
//!     b: HashMap<u64, as_holder!(B)>,
//! }
//!
//! #[derive(Debug, Clone, PartialEq, Holder)]
//! #[holder(table = Table, field = a)]
//! pub struct A {
//!     pub x: f64,
//!     pub y: f64,
//! }
//!
//! #[derive(Debug, Clone, PartialEq, Holder)]
//! #[holder(table = Table, field = b)]
//! pub struct B {
//!     pub z: f64,
//!     #[holder(use_place_holder)]
//!     pub a: A,
//! }
//! ```
//!
//! `#[derive(Holder)]` generates followings:
//!
//! - `AHolder` struct
//!   - naming rule is `{}Holder`
//!   - This name is obtained by `as_holder!(A)`
//! - `impl Holder for AHolder`
//!

use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{format_ident, quote};
use std::convert::*;

mod field_type;
mod for_struct;
mod holder_attr;

use field_type::*;
use holder_attr::*;

/// Generate `impl Deserialize` for entity structs
#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize_entry(input: TokenStream) -> TokenStream {
    derive_deserialize(&syn::parse(input).unwrap()).into()
}

fn derive_deserialize(ast: &syn::DeriveInput) -> TokenStream2 {
    let ident = &ast.ident;
    let visitor_ident = format_ident!("{}Visitor", ident);
    let name = ident.to_string().to_screaming_snake_case();
    match &ast.data {
        syn::Data::Struct(st) => {
            let fields: Vec<_> = st
                .fields
                .iter()
                .map(|f| {
                    f.ident
                        .as_ref()
                        .expect("Tuple struct case is not supported")
                })
                .collect();
            let attr_len = fields.len();
            quote! {
                #[automatically_derived]
                impl<'de> de::Deserialize<'de> for #ident {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: de::Deserializer<'de>,
                    {
                        deserializer.deserialize_tuple_struct(#name, #attr_len, #visitor_ident {})
                    }
                }

                #[doc(hidden)]
                struct #visitor_ident;

                #[automatically_derived]
                impl<'de> ::serde::de::Visitor<'de> for #visitor_ident {
                    type Value = #ident;
                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(formatter, #name)
                    }

                    fn visit_seq<A>(self, mut seq: A) -> ::std::result::Result<Self::Value, A::Error>
                    where
                        A: ::serde::de::SeqAccess<'de>,
                    {
                        if let Some(size) = seq.size_hint() {
                            if size != #attr_len {
                                todo!("Create another error and send it")
                            }
                        }
                        #( let #fields = seq.next_element()?.unwrap(); )*
                        Ok(#ident { #(#fields),* })
                    }

                    // Entry point for Record or Parameter::Typed
                    fn visit_map<A>(self, mut map: A) -> ::std::result::Result<Self::Value, A::Error>
                    where
                        A: ::serde::de::MapAccess<'de>,
                    {
                        let key: String = map
                            .next_key()?
                            .expect("Empty map cannot be accepted as ruststep Holder"); // this must be a bug, not runtime error
                        if key != #name {
                            todo!("Create Error type and send it")
                        }
                        let value = map.next_value()?; // send to Self::visit_seq
                        Ok(value)
                    }
                }
            } // quote!
        }
        _ => unimplemented!("Only struct is supprted currently"),
    }
}

#[proc_macro_derive(Holder, attributes(holder))]
pub fn derive_holder_entry(input: TokenStream) -> TokenStream {
    derive_holder(&syn::parse(input).unwrap()).into()
}

fn derive_holder(ast: &syn::DeriveInput) -> TokenStream2 {
    let table_attr = parse_table_attr(ast);
    let ident = &ast.ident;
    match &ast.data {
        syn::Data::Struct(st) => {
            let def_holder_tt = for_struct::def_holder(ident, st);
            let impl_holder_tt = for_struct::impl_holder(ident, &table_attr, st);
            let impl_entity_table_tt = for_struct::impl_entity_table(ident, &table_attr);
            quote! {
                #def_holder_tt
                #impl_holder_tt
                #impl_entity_table_tt
            }
        }
        _ => unimplemented!("Only struct is supprted currently"),
    }
}

/// Resolve Holder struct from owned type, e.g. `A` to `AHolder`
#[proc_macro]
pub fn as_holder(input: TokenStream) -> TokenStream {
    let path = as_holder_path(syn::parse(input).unwrap());
    let ts = quote! { #path };
    ts.into()
}

fn as_holder_ident(input: &syn::Ident) -> syn::Ident {
    quote::format_ident!("{}Holder", input)
}

fn as_holder_path(input: syn::Type) -> syn::Type {
    let ft: FieldType = input
        .try_into()
        .expect("as_holder! only accepts espr-generated type");
    ft.as_holder().into()
}

/// Returns `crate` or `::ruststep` as in ruststep crate or not
fn ruststep_crate() -> syn::Path {
    let path = crate_name("ruststep").unwrap();
    match path {
        FoundCrate::Itself => {
            let mut segments = syn::punctuated::Punctuated::new();
            segments.push(syn::PathSegment {
                ident: syn::Ident::new("crate", Span::call_site()),
                arguments: syn::PathArguments::None,
            });
            syn::Path {
                leading_colon: None,
                segments,
            }
        }
        FoundCrate::Name(name) => {
            let mut segments = syn::punctuated::Punctuated::new();
            segments.push(syn::PathSegment {
                ident: syn::Ident::new(&name, Span::call_site()),
                arguments: syn::PathArguments::None,
            });
            syn::Path {
                leading_colon: Some(syn::token::Colon2::default()),
                segments,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn holder_path() {
        let path = syn::parse_str("::some::Struct").unwrap();
        let holder = as_holder_path(path);
        let ans = syn::parse_str("::some::StructHolder").unwrap();
        assert_eq!(holder, ans);
    }

    #[test]
    fn optional_holder_path() {
        let path = syn::parse_str("Option<::some::Struct>").unwrap();
        let holder = as_holder_path(path);
        let ans = syn::parse_str("Option<::some::StructHolder>").unwrap();
        assert_eq!(holder, ans);
    }
}
