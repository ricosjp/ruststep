//! Generate Rust code using proc-macro utility crates

mod entity;
mod schema;

use crate::semantics::*;

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
            pub type #id = #ty;
        });
    }
}

impl ToTokens for Rename {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = format_ident!("{}", &self.id.to_pascal_case());
        let ty = &self.ty;
        tokens.append_all(quote! {
            pub type #id = #ty;
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
            #[derive(Debug, Clone)]
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
        for ty in &self.types {
            match ty {
                TypeRef::Entity {
                    name,
                    has_supertype_decl,
                    ..
                } => {
                    let name = format_ident!("{}", name.to_pascal_case());
                    entries.push(quote! { #name });
                    if *has_supertype_decl {
                        // avoid Box<Box<XxxAny>>
                        entry_types.push(quote! { #ty });
                    } else {
                        entry_types.push(quote! { Box<#ty> });
                    }
                }
                _ => {
                    entries.push(ty.to_token_stream());
                    entry_types.push(quote! { Box<#ty> });
                }
            }
        }
        tokens.append_all(quote! {
            #[derive(Debug, Clone)]
            pub enum #id {
                #(#entries(#entry_types)),*
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
        use crate::ast::types::SimpleType::*;
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
                name,
                has_supertype_decl,
                ..
            } => {
                if *has_supertype_decl {
                    let name = format_ident!("{}Any", name.to_pascal_case());
                    tokens.append_all(quote! { Box<dyn #name> });
                } else {
                    let name = format_ident!("{}", name.to_pascal_case());
                    tokens.append_all(quote! { #name });
                }
            }
            Set { base, .. } | List { base, .. } => {
                tokens.append_all(quote! { Vec<#base> });
            }
        }
    }
}
