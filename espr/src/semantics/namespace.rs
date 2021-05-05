use super::{scope::*, type_ref::*, SemanticError};
use crate::ast::{self, SyntaxTree};
use std::collections::HashMap;

/// Names in each scopes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Names {
    /// Declared as `SCHEMA`
    schemas: Vec<String>,
    /// Declared as `ENTITY`
    entities: Vec<(String, bool /* has supertype decl */)>,
    /// Declared as an attribute
    attributes: Vec<String>,

    /// Renames of primitive type, e.g. `TYPE label = STRING; ENDTYPE;`
    simple_types: Vec<String>,
    /// Renames of user defined type,
    /// e.g. `TYPE box_height = positive_ratio_measure; END_TYPE;`
    rename_types: Vec<String>,
    /// Enumeration of values,
    /// e.g. `TYPE text_path = ENUMERATION OF (up, right, down, left); END_TYPE;`
    enumeration_types: Vec<String>,
    /// Select of user defined types,
    /// e.g. `TYPE geometric_set_select = SELECT (point, curve); END_TYPE;`
    select_types: Vec<String>,
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
            let mut simple_types = Vec::new();
            let mut rename_types = Vec::new();
            let mut enumeration_types = Vec::new();
            let mut select_types = Vec::new();
            for ty in &schema.types {
                use ast::types::UnderlyingType;
                match ty.underlying_type {
                    UnderlyingType::Simple(..) => simple_types.push(ty.type_id.clone()),
                    UnderlyingType::Reference(..) => rename_types.push(ty.type_id.clone()),
                    UnderlyingType::Enumeration { .. } => {
                        enumeration_types.push(ty.type_id.clone())
                    }
                    UnderlyingType::Select { .. } => select_types.push(ty.type_id.clone()),
                    _ => unimplemented!(),
                }
            }
            names.insert(
                current_scope.clone(),
                Names {
                    schemas: Vec::new(),
                    entities: schema
                        .entities
                        .iter()
                        .map(|e| (e.name.clone(), e.has_supertype_decl()))
                        .collect(),
                    attributes: Vec::new(),
                    simple_types,
                    rename_types,
                    enumeration_types,
                    select_types,
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
            for (entity_name, has_supertype_decl) in &ns.entities {
                if name == entity_name {
                    return Ok(TypeRef::Entity {
                        name: name.to_string(),
                        scope,
                        has_supertype_decl: *has_supertype_decl,
                    });
                }
            }
            for ty in ns
                .simple_types
                .iter()
                .chain(ns.rename_types.iter())
                .chain(ns.enumeration_types.iter())
                .chain(ns.select_types.iter())
            {
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
