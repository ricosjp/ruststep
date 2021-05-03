use super::{scope::*, type_ref::*, SemanticError};
use crate::ast::{self, SyntaxTree};
use std::collections::HashMap;

/// Names in each scopes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Names {
    /// Declared as `SCHEMA`
    schemas: Vec<String>,
    /// Declared as `ENTITY`
    entities: Vec<(String, bool /* is supertype */)>,
    /// Declared as an attribute
    attributes: Vec<String>,
    /// Declared as `TYPE`
    types: Vec<String>,
}

impl Names {
    fn from_schemas(schemas: impl Iterator<Item = String>) -> Self {
        let mut names: Names = Default::default();
        for schema in schemas {
            names.schemas.push(schema)
        }
        names
    }

    fn from_attributes(attrs: impl Iterator<Item = String>) -> Self {
        let mut names: Names = Default::default();
        for attr in attrs {
            names.attributes.push(attr)
        }
        names
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Namespace(HashMap<Scope, Names>);

impl Namespace {
    pub fn new(syn: &SyntaxTree) -> Result<Self, SemanticError> {
        let mut names = HashMap::new();
        let mut current_scope = Scope::root();

        // All components except for schema must be in schema
        names.insert(
            current_scope.clone(),
            Names::from_schemas(syn.schemas.iter().map(|schema| schema.name.clone())),
        );

        for schema in &syn.schemas {
            current_scope = current_scope.pushed(ScopeType::Schema, &schema.name);
            names.insert(
                current_scope.clone(),
                Names {
                    schemas: Vec::new(),
                    entities: schema
                        .entities
                        .iter()
                        .map(|e| {
                            // FIXME This assumes that `SUPERTYPE` declaration exists for all
                            // supertypes. There is no guarantee for it generally.
                            let name = e.name.clone();
                            if let Some(c) = &e.constraint {
                                use ast::entity::Constraint;
                                match c {
                                    Constraint::AbstractSuperType(..)
                                    | Constraint::SuperTypeRule(..) => (name, true),
                                    Constraint::AbstractEntity => (name, false),
                                }
                            } else {
                                (name, false)
                            }
                        })
                        .collect(),
                    attributes: Vec::new(),
                    types: schema.types.iter().map(|e| e.type_id.clone()).collect(),
                },
            );

            for entity in &schema.entities {
                current_scope = current_scope.pushed(ScopeType::Entity, &entity.name);
                let attr_names =
                    Names::from_attributes(entity.attributes.iter().map(|attr| match &attr.name {
                        ast::entity::AttributeDecl::Reference(name) => name.clone(),
                        _ => unimplemented!(),
                    }));
                names.insert(current_scope.clone(), attr_names);
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
            for (entity_name, is_supertype) in &ns.entities {
                if name == entity_name {
                    return Ok(TypeRef::Entity {
                        name: name.to_string(),
                        scope,
                        is_supertype: *is_supertype,
                    });
                }
            }
            for ty in &ns.types {
                if name == ty {
                    return Ok(TypeRef::Named {
                        name: ty.to_string(),
                        scope,
                    });
                }
            }
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
