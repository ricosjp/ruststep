use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

use crate::ir::*;

impl ToTokens for Enumeration {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = format_ident!("{}", &self.id.to_pascal_case());
        let items: Vec<_> = self
            .items
            .iter()
            .map(|i| format_ident!("{}", i.to_pascal_case()))
            .collect();
        tokens.append_all(quote! {
            #[derive(Debug, Clone, PartialEq, ::serde::Deserialize)]
            pub enum #id {
                #( #items ),*
            }
        });
    }
}
