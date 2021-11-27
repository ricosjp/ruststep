use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

use crate::ir::*;

impl ToTokens for TypeRef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use TypeRef::*;
        match self {
            SimpleType(ty) => ty.to_tokens(tokens),
            Named { name, .. } => {
                let name = format_ident!("{}", name.to_pascal_case());
                tokens.append_all(quote! { #name });
            }
            Entity {
                name, is_supertype, ..
            } => {
                let name = if *is_supertype {
                    format_ident!("{}Any", name.to_pascal_case())
                } else {
                    format_ident!("{}", name.to_pascal_case())
                };
                tokens.append_all(quote! { #name });
            }
            Set { base, .. } | List { base, .. } => {
                tokens.append_all(quote! { Vec<#base> });
            }
        }
    }
}
