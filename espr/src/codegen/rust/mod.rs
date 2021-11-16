//! Generate Rust code using proc-macro utility crates

mod entity;
mod schema;

use crate::ir::*;

use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

impl ToTokens for IR {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let schemas = &self.schemas;
        tokens.append_all(quote! {
            #(#schemas)*
        })
    }
}

impl ToTokens for Simple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = format_ident!("{}", &self.id.to_pascal_case());
        let ty = &self.ty;
        tokens.append_all(quote! {
            #[derive(Clone, Debug, PartialEq, AsRef, Deref, DerefMut, ::serde::Serialize, ::serde::Deserialize)]
            pub struct #id(#ty);
        });
    }
}

impl ToTokens for Rename {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = format_ident!("{}", &self.id.to_pascal_case());
        let ty = &self.ty;
        tokens.append_all(quote! {
            #[derive(Clone, Debug, PartialEq, AsRef, Deref, DerefMut, ::serde::Serialize, ::serde::Deserialize)]
            pub struct #id(#ty);
        });
    }
}

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

impl ToTokens for Select {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = format_ident!("{}", &self.id.to_pascal_case());
        let mut entries = Vec::new();
        let mut entry_types = Vec::new();
        let mut field = Vec::new();
        let mut use_place_holder = Vec::new();
        for ty in &self.types {
            match ty {
                TypeRef::Entity {
                    name, is_supertype, ..
                } => {
                    entries.push(format_ident!("{}", name.to_pascal_case()));
                    if *is_supertype {
                        entry_types.push(quote! { #ty });
                        field.push(quote! {});
                    } else {
                        entry_types.push(quote! { Box<#ty> });
                        let field_name = format_ident!("{}", name);
                        field.push(quote! { #[holder(field = #field_name)] })
                    }
                    use_place_holder.push(quote! { #[holder(use_place_holder)] });
                }
                TypeRef::Named {
                    name, is_simple, ..
                } => {
                    field.push(quote! {});
                    entries.push(format_ident!("{}", name.to_pascal_case()));
                    if *is_simple {
                        entry_types.push(quote! { #ty });
                        use_place_holder.push(quote! {});
                    } else {
                        entry_types.push(quote! { Box<#ty> });
                        use_place_holder.push(quote! { #[holder(use_place_holder)] });
                    }
                }
                _ => unimplemented!(),
            }
        }
        tokens.append_all(quote! {
            #[derive(Debug, Clone, PartialEq, ::ruststep_derive::Holder)]
            #[holder(table = Tables)]
            #[holder(generate_deserialize)]
            pub enum #id {
                #(
                #field
                #use_place_holder
                #entries(#entry_types)
                ),*
            }
        });
    }
}

impl ToTokens for TypeDecl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TypeDecl::Simple(simple) => simple.to_tokens(tokens),
            TypeDecl::Rename(rename) => rename.to_tokens(tokens),
            TypeDecl::Enumeration(e) => e.to_tokens(tokens),
            TypeDecl::Select(select) => select.to_tokens(tokens),
        }
    }
}

impl ToTokens for SimpleType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use crate::ast::SimpleType::*;
        match self.0 {
            Number => tokens.append(format_ident!("f64")),
            Real => tokens.append(format_ident!("f64")),
            Integer => tokens.append(format_ident!("i64")),
            Logical => tokens.append_all(quote! { Logical }),
            Boolen => tokens.append(format_ident!("bool")),
            String_ { .. } => tokens.append(format_ident!("String")),
            Binary { .. } => unimplemented!("Binary type is not supported yet"),
        }
    }
}

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
