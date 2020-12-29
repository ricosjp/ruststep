use super::{scope::*, SemanticError};
use crate::{
    error::*,
    parser::{self, SyntaxTree},
};
use maplit::hashmap;
use proc_macro2::TokenStream;
use quote::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IdentifierType {
    Entity,
    Schema,
    Attribute,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeRef {
    Named { name: String, scope: Scope },
    SimpleType(parser::simple_data_type::SimpleType),
}

impl ToTokens for TypeRef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use TypeRef::*;
        match self {
            SimpleType(ty) => {
                use parser::simple_data_type::SimpleType::*;
                match ty {
                    Number => tokens.append(format_ident!("f64")),
                    Real => tokens.append(format_ident!("f64")),
                    Integer => tokens.append(format_ident!("i64")),
                    Logical => tokens.append_all(quote! { ::espr_runtime::Logial }),
                    Boolen => tokens.append(format_ident!("bool")),
                    _ => unimplemented!(),
                }
            }
            Named { name, scope } => tokens.append_all(quote! { #scope #name }),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Namespace(HashMap<Scope, HashMap<IdentifierType, Vec<String>>>);

impl Namespace {
    pub fn new(syn: &SyntaxTree) -> Result<Self, Error> {
        let mut names = HashMap::new();
        let mut current_scope = Scope::root();
        names.insert(
            current_scope.clone(),
            hashmap! {
                IdentifierType::Schema => syn.schemas.iter().map(|schema| schema.name.clone()).collect()
            },
        );

        for schema in &syn.schemas {
            current_scope = current_scope.pushed(ScopeType::Schema, &schema.name);
            names.insert(
                current_scope.clone(),
                hashmap! {
                    IdentifierType::Entity => schema.entities.iter().map(|e| e.name.clone()).collect()
                },
            );

            for entity in &schema.entities {
                current_scope = current_scope.pushed(ScopeType::Entity, &entity.name);
                let attrs = entity
                    .attributes
                    .iter()
                    .map(|(name, _ty)| name.clone())
                    .collect();
                names.insert(
                    current_scope.clone(),
                    hashmap! {
                        IdentifierType::Attribute => attrs
                    },
                );
                current_scope = current_scope.popped().expect("Never be root");
            }
            current_scope = current_scope.popped().expect("Never be root");
        }
        Ok(Self(names))
    }

    /// Panics
    /// -------
    /// - when `scope` is not belongs to this Namespace
    pub fn lookup_type(&self, scope: &Scope, name: &str) -> Result<TypeRef, SemanticError> {
        let mut scope = scope.clone();
        loop {
            let ns = self
                .0
                .get(&scope)
                .expect("Scope is not belong to the namespace");
            if let Some(entities) = ns.get(&IdentifierType::Entity) {
                for entity_name in entities {
                    if name == entity_name {
                        return Ok(TypeRef::Named {
                            name: name.to_string(),
                            scope: scope,
                        });
                    }
                }
            } // skip if entity does not exist
            if let Some(popped) = scope.popped() {
                scope = popped;
            } else {
                break;
            }
        }
        Err(SemanticError::TypeNotFound {
            name: name.to_string(),
            scope: scope.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn namespace() {
        let ns = Namespace::new(&SyntaxTree::example());
        dbg!(&ns);
    }
}
