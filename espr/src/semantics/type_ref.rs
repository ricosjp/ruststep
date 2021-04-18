use super::{namespace::*, scope::*, *};
use crate::parser;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bound {}

impl Legalize for Bound {
    type Input = parser::Bound;
    fn legalize(
        _ns: &Namespace,
        _scope: &Scope,
        _input: &Self::Input,
    ) -> Result<Self, SemanticError> {
        // FIXME
        Ok(Bound {})
    }
}

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
    List {
        ty: Box<TypeRef>,
        bound_spec: Option<Bound>,
        unique: bool,
    },
}

impl Legalize for TypeRef {
    type Input = parser::ParameterType;

    fn legalize(
        ns: &Namespace,
        scope: &Scope,
        ty: &parser::ParameterType,
    ) -> Result<Self, SemanticError> {
        use parser::ParameterType::*;
        Ok(match ty {
            Simple(ty) => Self::SimpleType(*ty),
            Named(name) => ns.lookup_type(scope, name)?,
            Set { ty, bound_spec } => {
                let ty = TypeRef::legalize(ns, scope, ty.as_ref())?;
                let bound_spec = if let Some(bound_spec) = bound_spec {
                    Some(Legalize::legalize(ns, scope, bound_spec)?)
                } else {
                    None
                };
                Self::Set {
                    ty: Box::new(ty),
                    bound_spec,
                }
            }
            List {
                ty,
                bound_spec,
                unique,
            } => {
                let ty = TypeRef::legalize(ns, scope, ty.as_ref())?;
                let bound_spec = if let Some(bound_spec) = bound_spec {
                    Some(Legalize::legalize(ns, scope, bound_spec)?)
                } else {
                    None
                };
                Self::List {
                    ty: Box::new(ty),
                    bound_spec,
                    unique: *unique,
                }
            }
            _ => todo!(),
        })
    }
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
                tokens.append_all(quote! { #name })
            }
            Set { ty, .. } | List { ty, .. } => {
                ty.to_tokens(tokens);
            }
        }
    }
}
