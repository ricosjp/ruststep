use super::scope::*;
use crate::parser;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bound {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeRef {
    Named {
        name: String,
        scope: Scope,
    },
    SimpleType(parser::SimpleType),

    Set {
        ty: Box<TypeRef>,
        bound_spec: Option<Bound>,
    },
    Bag {
        ty: Box<TypeRef>,
        bound_spec: Option<Bound>,
    },
    List {
        ty: Box<TypeRef>,
        bound_spec: Option<Bound>,
        unique: bool,
    },
    Array {
        ty: Box<TypeRef>,
        bound_spec: Option<Bound>,
        unique: bool,
        optional: bool,
    },
    Aggregate {
        ty: Box<TypeRef>,
        label: Option<String>,
    },
    GenericEntity(Option<String>),
    Generic(Option<String>),
}

impl ToTokens for TypeRef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use TypeRef::*;
        match self {
            SimpleType(ty) => {
                use parser::SimpleType::*;
                match ty {
                    Number => tokens.append(format_ident!("f64")),
                    Real => tokens.append(format_ident!("f64")),
                    Integer => tokens.append(format_ident!("i64")),
                    Logical => tokens.append_all(quote! { ::espr_runtime::Logical }),
                    Boolen => tokens.append(format_ident!("bool")),
                    String_ { .. } => tokens.append(format_ident!("String")),
                    Binary { .. } => unimplemented!("Binary type is not supported yet"),
                }
            }
            Named { name, .. } => {
                let name = format_ident!("{}", name.to_pascal_case());
                // FIXME This type name should be full path like
                //
                // ```
                // tokens.append_all(quote! { #scope :: #name })
                // ```
                //
                // But it does not work as desired.
                // See https://gitlab.ritc.jp/ricos/truck/ruststep/-/merge_requests/12/diffs#note_14506
                tokens.append_all(quote! { #name })
            }
            _ => unimplemented!(),
        }
    }
}
