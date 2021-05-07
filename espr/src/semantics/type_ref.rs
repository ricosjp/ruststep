use super::{namespace::*, scope::*, *};
use crate::ast;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleType(pub ast::types::SimpleType);

impl Legalize for SimpleType {
    type Input = ast::types::SimpleType;
    fn legalize(
        _ns: &Namespace,
        _scope: &Scope,
        input: &Self::Input,
    ) -> Result<Self, SemanticError> {
        Ok(SimpleType(input.clone()))
    }
}

impl ToTokens for SimpleType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use ast::types::SimpleType::*;
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bound {}

impl Legalize for Bound {
    type Input = ast::types::Bound;
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
    SimpleType(SimpleType),
    Named {
        name: String,

        /// Scope where the named type is declared
        scope: Scope,

        /// True if the underlying type of named type is a simple type:
        ///
        /// ```text
        /// TYPE a = INTEGER; ENDTYPE;
        /// TYPE b = a; ENDTYPE;
        /// ```
        ///
        /// Then both `a` and `b` are simple.
        ///
        is_simple: bool,
    },

    /* Declared as `ENTITY` */
    Entity {
        name: String,
        scope: Scope,
        has_supertype_decl: bool,
    },

    /* Aggregated */
    Set {
        base: Box<TypeRef>,
        bound: Option<Bound>,
    },
    List {
        base: Box<TypeRef>,
        bound: Option<Bound>,
        unique: bool,
    },
}

impl TypeRef {
    pub fn is_simple(&self) -> bool {
        match self {
            TypeRef::SimpleType(..) => true,
            TypeRef::Named { is_simple, .. } => *is_simple,
            _ => false,
        }
    }
}

impl Legalize for TypeRef {
    type Input = ast::types::ParameterType;

    fn legalize(
        ns: &Namespace,
        scope: &Scope,
        ty: &ast::types::ParameterType,
    ) -> Result<Self, SemanticError> {
        use ast::types::ParameterType::*;
        Ok(match ty {
            Simple(ty) => Self::SimpleType(SimpleType(*ty)),
            Named(name) => ns.lookup_type(scope, name)?,
            Set { base, bound } => {
                let base = TypeRef::legalize(ns, scope, base.as_ref())?;
                let bound = if let Some(bound) = bound {
                    Some(Legalize::legalize(ns, scope, bound)?)
                } else {
                    None
                };
                Self::Set {
                    base: Box::new(base),
                    bound,
                }
            }
            List {
                base,
                bound,
                unique,
            } => {
                let base = TypeRef::legalize(ns, scope, base.as_ref())?;
                let bound = if let Some(bound) = bound {
                    Some(Legalize::legalize(ns, scope, bound)?)
                } else {
                    None
                };
                Self::List {
                    base: Box::new(base),
                    bound,
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
