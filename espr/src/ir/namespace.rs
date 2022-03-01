use super::{scope::*, SemanticError};
use crate::ast::{self, SyntaxTree};

use std::{collections::HashMap, fmt};

/// Named AST portion of corresponding [Path]
#[derive(Debug, Clone, Copy)] // Copy since this is actually immutable reference
pub enum Named<'st> {
    Type(&'st ast::TypeDecl),
    Entity(&'st ast::Entity),
}

/// Namespace of loaded EXPRESS schema
///
/// This struct will be constructed at the first time of IR creation,
/// and is responsible for
///
/// - Resolving name in each [Scope] into [Path]
/// - Get a reference to AST portion corresponding to [Path]
///
#[derive(Clone)]
pub struct Namespace<'st> {
    pub names: HashMap<Scope, Vec<(ScopeType, String)>>,
    /// Indexed AST portion
    pub ast: Vec<(Path, Named<'st>)>,
}

impl fmt::Debug for Namespace<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[Namespace.names]")?;
        for (k, v) in &self.names {
            writeln!(f, "{} = {:>2?}", k, v)?;
        }
        writeln!(f, "[Namespace.ast]")?;
        for (k, v) in &self.ast {
            writeln!(f, "{} = {:>2?}", k, v)?;
        }
        Ok(())
    }
}

impl<'st> Namespace<'st> {
    pub fn new(st: &'st SyntaxTree) -> Self {
        let mut names = HashMap::new();
        let mut ast = Vec::new();
        let root = Scope::root();

        for schema in &st.schemas {
            let here = root.pushed(ScopeType::Schema, &schema.name);
            let mut current_names = Vec::new();
            for ty in &schema.types {
                let name = &ty.type_id;
                current_names.push((ScopeType::Type, name.to_string()));
                let path = Path::new(&here, ScopeType::Type, name);
                ast.push((path, Named::Type(ty)));
            }
            for entity in &schema.entities {
                let name = &entity.name;
                current_names.push((ScopeType::Entity, name.to_string()));
                let path = Path::new(&here, ScopeType::Entity, name);
                ast.push((path, Named::Entity(entity)));
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

    /// Get an AST portion corresponding the [Path]
    ///
    /// Error
    /// ------
    /// - Input path is invalid, i.e. No item is specified by the path.
    ///
    pub fn get(&self, path: &Path) -> Result<Named, SemanticError> {
        for (p, ast) in &self.ast {
            if p == path {
                return Ok(*ast);
            }
        }
        Err(SemanticError::InvalidPath(path.clone()))
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

    #[test]
    fn namespace_debug() {
        let st = ast::SyntaxTree::parse(
            r#"
            SCHEMA test_schema;
              ENTITY base SUPERTYPE OF (ONEOF (sub1, sub2));
                x: REAL;
              END_ENTITY;

              ENTITY sub1 SUBTYPE OF (base);
                y1: REAL;
              END_ENTITY;

              ENTITY sub2 SUBTYPE OF (base);
                y2: REAL;
              END_ENTITY;
            END_SCHEMA;
            "#,
        )
        .unwrap();
        let ns = Namespace::new(&st);

        insta::assert_snapshot!(format!("{:?}", ns), @r###"
        [Namespace.names]
        test_schema = [(Entity, "base"), (Entity, "sub1"), (Entity, "sub2")]
        [Namespace.ast]
        test_schema.base = Entity(Entity base
          EntityAttribute { name: Reference("x"), ty: Simple(Real), optional: false }
          SuperTypeRule(OneOf { exprs: [Reference("sub1"), Reference("sub2")] })
        )
        test_schema.sub1 = Entity(Entity sub1
          EntityAttribute { name: Reference("y1"), ty: Simple(Real), optional: false }
          SubTypeDecl { entity_references: ["base"] }
        )
        test_schema.sub2 = Entity(Entity sub2
          EntityAttribute { name: Reference("y2"), ty: Simple(Real), optional: false }
          SubTypeDecl { entity_references: ["base"] }
        )
        "###);
    }
}
