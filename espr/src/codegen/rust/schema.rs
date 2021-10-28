use crate::ir::*;

use check_keyword::CheckKeyword;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

impl ToTokens for Schema {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", self.name);
        let types = &self.types;
        let entities = &self.entities;
        let entity_types: Vec<_> = entities
            .iter()
            .flat_map(|e| {
                if e.subtypes.is_empty() {
                    vec![format_ident!("{}", e.name.to_pascal_case())]
                } else {
                    vec![
                        format_ident!("{}", e.name.to_pascal_case()),
                        format_ident!("{}Any", e.name.to_pascal_case(),),
                    ]
                }
            })
            .collect();
        let holder_name: Vec<_> = entities
            .iter()
            .flat_map(|e| {
                if e.subtypes.is_empty() {
                    vec![format_ident!("{}", e.name.to_safe())]
                } else {
                    vec![
                        format_ident!("{}", e.name.to_safe()),
                        format_ident!("{}_any", e.name),
                    ]
                }
            })
            .collect();
        let iter_name: Vec<_> = entities
            .iter()
            .flat_map(|e| {
                if e.subtypes.is_empty() {
                    vec![format_ident!("{}_iter", e.name)]
                } else {
                    vec![
                        format_ident!("{}_iter", e.name),
                        format_ident!("{}_any_iter", e.name),
                    ]
                }
            })
            .collect();
        tokens.append_all(quote! {
            pub mod #name {
                use crate::{primitive::*, tables::*, error::Result};
                use std::collections::HashMap;
                use ruststep_derive::as_holder;

                #[derive(Debug, Clone, PartialEq, Default)]
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
        });
    }
}
