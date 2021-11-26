use crate::ir::*;

use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

impl ToTokens for Rename {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field_name = format_ident!("{}", &self.id.to_snake_case());
        let id = format_ident!("{}", &self.id.to_pascal_case());
        let ty = &self.ty;
        let (derive, use_place_holder) = match ty {
            TypeRef::SimpleType(_) => simple_meta(),
            TypeRef::Named { is_simple, .. } => {
                if *is_simple {
                    simple_meta()
                } else {
                    rename_meta(&field_name)
                }
            }
            _ => rename_meta(&field_name),
        };

        tokens.append_all(quote! {
            #derive
            pub struct #id(#use_place_holder pub #ty);
        });
    }
}

fn simple_meta() -> (TokenStream, TokenStream) {
    (
        quote! {#[derive(Clone, Debug, PartialEq, AsRef, Deref, DerefMut, ::serde::Serialize, ::serde::Deserialize)]},
        quote! {},
    )
}

fn rename_meta(field_name: &syn::Ident) -> (TokenStream, TokenStream) {
    (
        quote! {
            #[derive(Clone, Debug, PartialEq, AsRef, Deref, DerefMut, ::ruststep_derive::Holder)]
            #[holder(table = Tables)]
            #[holder(field = #field_name)]
            #[holder(generate_deserialize)]
        },
        quote! {#[use_place_holder]},
    )
}
