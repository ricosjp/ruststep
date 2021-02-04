use super::{scope::*, SemanticError};
use crate::parser::{self, syntax_tree::SyntaxTree};
use inflector::Inflector;
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
    SimpleType(parser::types::SimpleType),
}

impl ToTokens for TypeRef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use TypeRef::*;
        match self {
            SimpleType(ty) => {
                use parser::types::SimpleType::*;
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
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Namespace(HashMap<Scope, HashMap<IdentifierType, Vec<String>>>);

impl Namespace {
    pub fn new(syn: &SyntaxTree) -> Result<Self, SemanticError> {
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
                    .map(|attr| match &attr.name {
                        parser::entity::AttributeDecl::Reference(name) => name.clone(),
                        _ => unimplemented!(),
                    })
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
