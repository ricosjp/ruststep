use super::*;
use proc_macro2::TokenStream;
use quote::*;

/// Parsed result of EXPRESS's ENTITY
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    /// Name of this entity type
    pub name: String,

    /// attribute name and types
    ///
    /// Be sure that this "type" is a string, not validated type in this timing
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    name: String,
    ty: Type,
    optional: bool,
}

impl ToTokens for Entity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", self.name);
        let member_name: Vec<_> = self
            .attributes
            .iter()
            .map(|Attribute { name, .. }| format_ident!("{}", name))
            .collect();
        let member_type: Vec<_> = self
            .attributes
            .iter()
            .map(|Attribute { ty, optional, .. }| {
                if *optional {
                    quote! { Option<#ty> }
                } else {
                    quote! { #ty }
                }
            })
            .collect();

        tokens.append_all(quote! {
            #[derive(Clone, Debug, PartialEq)]
            pub struct #name {
                #(
                #member_name : #member_type,
                )*
            }

            impl #name {
                pub fn new(#(#member_name : #member_type),*) -> Self {
                    Self { #(#member_name),* }
                }
            }
        })
    }
}
