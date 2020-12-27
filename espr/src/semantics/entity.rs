use super::*;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    name: String,
    attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    name: String,
    ty: Type,
    optional: bool,
}

impl ToTokens for Entity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // EXPRESS identifier should be snake_case, but Rust struct should be PascalCase.
        let name = format_ident!("{}", self.name.to_pascal_case());

        let attr_name: Vec<_> = self
            .attributes
            .iter()
            .map(|Attribute { name, .. }| format_ident!("{}", name))
            .collect();
        let attr_type: Vec<_> = self
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
                #attr_name : #attr_type,
                )*
            }

            impl #name {
                pub fn new(#(#attr_name : #attr_type),*) -> Self {
                    Self { #(#attr_name),* }
                }
            }
        })
    }
}
