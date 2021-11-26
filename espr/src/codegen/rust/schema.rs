use crate::ir::*;

use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CratePrefix {
    Internal,
    External,
}

impl CratePrefix {
    pub fn as_path(&self) -> syn::Path {
        match self {
            CratePrefix::Internal => syn::parse_str("crate").unwrap(),
            CratePrefix::External => syn::parse_str("::ruststep").unwrap(),
        }
    }
}

impl IR {
    pub fn to_token_stream(&self, prefix: CratePrefix) -> TokenStream {
        let schemas: Vec<_> = self
            .schemas
            .iter()
            .map(|schema| schema.to_token_stream(prefix))
            .collect();
        quote! { #(#schemas)* }
    }
}

impl Schema {
    pub fn to_token_stream(&self, prefix: CratePrefix) -> TokenStream {
        let name = format_ident!("{}", self.name);
        let types = &self.types;
        let entities = &self.entities;
        let entity_types: Vec<_> = entities
            .iter()
            .map(|e| format_ident!("{}", e.name.to_pascal_case()))
            .collect();
        let holder_name: Vec<_> = entities
            .iter()
            .map(|e| format_ident!("{}", e.name))
            .collect();
        let iter_name: Vec<_> = entities
            .iter()
            .map(|e| format_ident!("{}_iter", e.name))
            .collect();

        let ruststep_path = prefix.as_path();

        quote! {
            pub mod #name {
                use #ruststep_path::{as_holder, Holder, TableInit, primitive::*, tables::*, error::Result};
                use std::collections::HashMap;
                use derive_more::*;

                #[derive(Debug, Clone, PartialEq, Default, TableInit)]
                pub struct Tables {
                    #(
                    #holder_name: HashMap<u64, as_holder!(#entity_types)>,
                    )*
                }

                impl Tables {
                    #(
                    pub fn #iter_name<'table>(&'table self) ->
                        impl Iterator<Item = Result<#entity_types>> + 'table
                    {
                        self.#holder_name
                            .values()
                            .cloned()
                            .map(move |value| value.into_owned(&self))
                    }
                    )*
                }

                #(#types)*
                #(#entities)*
            }
        }
    }
}
