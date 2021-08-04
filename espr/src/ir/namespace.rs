use super::{scope::*, SemanticError};
use crate::ast::{self, SyntaxTree};
use std::collections::HashMap;

/// Named AST portion of corresponding [Path]
#[derive(Debug, Clone, Copy)] // Copy since this is actually immutable reference
pub enum Named<'st> {
    Type(&'st ast::TypeDecl),
    Entity(&'st ast::Entity),
}

#[derive(Debug, Clone)]
pub struct Namespace<'st> {
    pub names: HashMap<Scope, Vec<(ScopeType, String)>>,
    pub ast: HashMap<Path, Named<'st>>,
}

impl<'st> Namespace<'st> {
    pub fn new(st: &'st SyntaxTree) -> Self {
        let mut names = HashMap::new();
        let mut ast = HashMap::new();
        let root = Scope::root();

        for schema in &st.schemas {
            let here = root.pushed(ScopeType::Schema, &schema.name);
            let mut current_names = Vec::new();
            for ty in &schema.types {
                let name = &ty.type_id;
                current_names.push((ScopeType::Type, name.to_string()));
                let path = Path::new(&here, ScopeType::Type, name);
                ast.insert(path, Named::Type(ty));
            }
            for entity in &schema.entities {
                let name = &entity.name;
                current_names.push((ScopeType::Entity, name.to_string()));
                let path = Path::new(&here, ScopeType::Entity, name);
                ast.insert(path, Named::Entity(entity));
            }
            names.insert(here, current_names);
        }

        Namespace { names, ast }
    }

    /// Resolve a `name` referred in a `scope` into the full path.
    ///
    /// Error
    /// ------
    /// - If no corresponding definition found.
    ///
    pub fn resolve(&self, scope: &Scope, name: &str) -> Result<Path, SemanticError> {
        let mut scope = scope.clone();
        loop {
            if let Some(names) = self.names.get(&scope) {
                for (ty, n) in names {
                    if name == n {
                        return Ok(Path::new(&scope, *ty, n));
                    }
                }
            }
            scope = scope.popped().ok_or_else(|| SemanticError::TypeNotFound {
                scope: scope.clone(),
                name: name.to_string(),
            })?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ns_init() {
        let st = SyntaxTree::parse(
            r#"
            SCHEMA one;
              ENTITY first;
                m_ref : second;
                fattr : STRING;
              END_ENTITY;
              ENTITY second;
                sattr : STRING;
              END_ENTITY;
            END_SCHEMA;

            SCHEMA geometry0;
              ENTITY point;
                x, y, z: REAL;
              END_ENTITY;
            END_SCHEMA;
            "#
            .trim(),
        )
        .unwrap();
        let ns = Namespace::new(&st);

        assert_eq!(ns.names.len(), 2);
        let root = Scope::root();
        for (scope, names) in &ns.names {
            if scope == &root.pushed(ScopeType::Schema, "one") {
                assert_eq!(names.len(), 2);
            }
            if scope == &root.pushed(ScopeType::Schema, "geometry0") {
                assert_eq!(names.len(), 1);
            }
        }
    }
}
