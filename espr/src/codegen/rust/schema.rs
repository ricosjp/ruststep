use crate::ir::*;

use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

impl ToTokens for Schema {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", self.name);
        let types = &self.types;
        let entities = &self.entities;
        let type_decls = self.types.iter().filter(|e| match e {
            TypeDecl::Simple(_) => false,
            TypeDecl::Rename(Rename {
                ty: TypeRef::SimpleType(_),
                ..
            }) => false,
            TypeDecl::Rename(Rename {
                ty: TypeRef::Named { is_simple, .. },
                ..
            }) => !is_simple,
            _ => true,
        });
        let entity_types: Vec<_> = entities
            .iter()
            .map(|e| format_ident!("{}", e.name.to_pascal_case()))
            .chain(
                type_decls
                    .clone()
                    .map(|e| format_ident!("{}", e.id().to_pascal_case())),
            )
            .collect();
        let holder_name: Vec<_> = entities
            .iter()
            .map(|e| format_ident!("{}", e.name))
            .chain(type_decls.clone().map(|e| format_ident!("{}", e.id())))
            .collect();
        let iter_name: Vec<_> = entities
            .iter()
            .map(|e| format_ident!("{}_iter", e.name))
            .chain(type_decls.map(|e| format_ident!("{}_iter", e.id())))
            .collect();
        tokens.append_all(quote! {
            pub mod #name {
                use crate::{primitive::*, tables::*, error::Result};
                use std::collections::HashMap;
                use ruststep_derive::{as_holder, TableInit};
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
        });
    }
}
