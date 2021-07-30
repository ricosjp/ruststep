use super::{scope::*, type_ref::*, SemanticError};
use crate::ast::{self, SyntaxTree};
use std::collections::HashMap;

/// Named AST portion of corresponding [Path]
#[derive(Debug, Clone, Copy)] // Copy since this is actually immutable reference
pub enum Named<'st> {
    Type(&'st ast::TypeDecl),
    Entity(&'st ast::Entity),
}

#[derive(Debug, Clone)]
pub struct Ns<'st> {
    pub names: HashMap<Scope, Vec<String>>,
    pub ast: HashMap<Path, Named<'st>>,
}

impl<'st> Ns<'st> {
    pub fn new(st: &'st SyntaxTree) -> Self {
        let mut names = HashMap::new();
        let mut ast = HashMap::new();
        let root = Scope::root();

        for schema in &st.schemas {
            let here = root.pushed(ScopeType::Schema, &schema.name);
            let mut current_names = Vec::new();
            for ty in &schema.types {
                let name = &ty.type_id;
                current_names.push(name.to_string());
                let path = Path::new(&here, name);
                ast.insert(path, Named::Type(ty));
            }
            for entity in &schema.entities {
                let name = &entity.name;
                current_names.push(name.to_string());
                let path = Path::new(&here, name);
                ast.insert(path, Named::Entity(entity));
            }
            names.insert(here, current_names);
        }

        Ns { names, ast }
    }

    /// Resolve a `name` used in a `scope` to full pash.
    ///
    /// Error
    /// ------
    /// - If no corresponding definition found.
    pub fn resolve(&self, scope: &Scope, name: &str) -> Result<Path, SemanticError> {
        let err = || SemanticError::TypeNotFound {
            scope: scope.clone(),
            name: name.to_string(),
        };
        let mut scope = scope.clone();
        loop {
            let names = self.names.get(&scope).ok_or_else(err)?;
            for n in names {
                if name == n {
                    return Ok(Path::new(&scope, n));
                }
            }
            scope = scope.popped().ok_or_else(err)?;
        }
    }

    /// Get an AST portion corresponding the path
    ///
    /// Error
    /// ------
    /// - Input path is invalid, i.e. No item is specified by the path.
    pub fn get(&self, path: &Path) -> Result<Named, SemanticError> {
        Ok(*self
            .ast
            .get(path)
            .ok_or_else(|| SemanticError::InvalidPath(path.clone()))?)
    }
}

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
    rename_types: Vec<(String, String /* underlying type */)>,
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
                use ast::Type;
                match &ty.underlying_type {
                    Type::Simple(..) => simple_types.push(ty.type_id.clone()),
                    Type::Named(underlying_type) => {
                        rename_types.push((ty.type_id.clone(), underlying_type.clone()))
                    }
                    Type::Enumeration { .. } => enumeration_types.push(ty.type_id.clone()),
                    Type::Select { .. } => select_types.push(ty.type_id.clone()),
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
                        ast::AttributeDecl::Reference(name) => name.clone(),
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

            for ty in &ns.simple_types {
                if name == ty {
                    return Ok(TypeRef::Named {
                        name: ty.to_string(),
                        scope,
                        is_simple: true,
                    });
                }
            }

            for ty in ns.enumeration_types.iter().chain(ns.select_types.iter()) {
                if name == ty {
                    return Ok(TypeRef::Named {
                        name: ty.to_string(),
                        scope,
                        is_simple: false,
                    });
                }
            }

            for (ty, underlying_type) in &ns.rename_types {
                if name == ty {
                    let underlying_type = self.lookup_type(&scope, &underlying_type)?;
                    return Ok(TypeRef::Named {
                        name: ty.to_string(),
                        scope,
                        is_simple: underlying_type.is_simple(),
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
