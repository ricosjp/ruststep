use crate::ir::*;

use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

impl ToTokens for Schema {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", self.name);
        let types = &self.types;
        let entities = &self.entities;
        let entity_name: Vec<_> = entities
            .iter()
            .map(|e| format_ident!("{}", e.name.to_pascal_case()))
            .collect();
        let holder_name: Vec<_> = entities
            .iter()
            .map(|e| format_ident!("{}", e.name))
            .collect();
        let holder_type: Vec<_> = entities
            .iter()
            .map(|e| format_ident!("{}Holder", e.name.to_pascal_case()))
            .collect();
        let iter_name: Vec<_> = entities
            .iter()
            .map(|e| format_ident!("{}_iter", e.name))
            .collect();
        tokens.append_all(quote! {
            pub mod #name {
                use crate::{primitive::*, place_holder::*, tables::*, error::Result};
                use std::collections::HashMap;

                #[derive(Debug, Clone, PartialEq, Default)]
                pub struct Tables {
                    #(
                    #holder_name: HashMap<u64, #holder_type>,
                    )*
                }

                impl Tables {
                    #(
                    pub fn #iter_name<'table>(&'table self) ->
                        impl Iterator<Item = Result<#entity_name>> + 'table
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
