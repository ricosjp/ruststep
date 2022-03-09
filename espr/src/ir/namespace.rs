use super::{scope::*, SemanticError};
use crate::ast::{self, SyntaxTree};

use std::collections::HashMap;

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
#[derive(Debug, Clone)]
pub struct Namespace<'st> {
    pub names: HashMap<Scope, Vec<(ScopeType, String, usize)>>,
    /// Indexed AST portion
    pub ast: Vec<(Path, Named<'st>)>,
}

impl<'st> std::ops::Index<usize> for Namespace<'st> {
    type Output = (Path, Named<'st>);
    fn index(&self, id: usize) -> &Self::Output {
        &self.ast[id]
    }
}

impl<'st> std::ops::Index<&Scope> for Namespace<'st> {
    type Output = [(ScopeType, String, usize)];
    fn index(&self, id: &Scope) -> &Self::Output {
        &self.names[id]
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
                let path = Path::new(&here, ScopeType::Type, name);
                ast.push((path, Named::Type(ty)));
                let index = ast.len();
                current_names.push((ScopeType::Type, name.to_string(), index));
            }
            for entity in &schema.entities {
                let name = &entity.name;
                let path = Path::new(&here, ScopeType::Entity, name);
                ast.push((path, Named::Entity(entity)));
                let index = ast.len();
                current_names.push((ScopeType::Entity, name.to_string(), index));
            }
            names.insert(here, current_names);
        }

        Namespace { names, ast }
    }

    /// Size of indexed AST
    pub fn len(&self) -> usize {
        self.ast.len()
    }

    /// Resolve a `name` referred in a `scope` into the full path.
    ///
    /// Error
    /// ------
    /// - If no corresponding definition found.
    ///
    pub fn resolve(&self, scope: &Scope, name: &str) -> Result<(Path, usize), SemanticError> {
        let mut scope = scope.clone();
        loop {
            if let Some(names) = self.names.get(&scope) {
                for (ty, n, index) in names {
                    if name == n {
                        return Ok((Path::new(&scope, *ty, n), *index));
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

        insta::assert_snapshot!(format!("{:#?}", ns), @r###"
        Namespace {
            names: {
                Scope(test_schema[Schema]): [
                    (
                        Entity,
                        "base",
                    ),
                    (
                        Entity,
                        "sub1",
                    ),
                    (
                        Entity,
                        "sub2",
                    ),
                ],
            },
            ast: [
                (
                    Scope(test_schema[Schema]).base[Entity],
                    Entity(
                        Entity {
                            name: "base",
                            attributes: [
                                EntityAttribute {
                                    name: Reference(
                                        "x",
                                    ),
                                    ty: Simple(
                                        Real,
                                    ),
                                    optional: false,
                                },
                            ],
                            constraint: Some(
                                SuperTypeRule(
                                    OneOf {
                                        exprs: [
                                            Reference(
                                                "sub1",
                                            ),
                                            Reference(
                                                "sub2",
                                            ),
                                        ],
                                    },
                                ),
                            ),
                            subtype_of: None,
                            derive_clause: None,
                            inverse_clause: None,
                            unique_clause: None,
                            where_clause: None,
                        },
                    ),
                ),
                (
                    Scope(test_schema[Schema]).sub1[Entity],
                    Entity(
                        Entity {
                            name: "sub1",
                            attributes: [
                                EntityAttribute {
                                    name: Reference(
                                        "y1",
                                    ),
                                    ty: Simple(
                                        Real,
                                    ),
                                    optional: false,
                                },
                            ],
                            constraint: None,
                            subtype_of: Some(
                                SubTypeDecl {
                                    entity_references: [
                                        "base",
                                    ],
                                },
                            ),
                            derive_clause: None,
                            inverse_clause: None,
                            unique_clause: None,
                            where_clause: None,
                        },
                    ),
                ),
                (
                    Scope(test_schema[Schema]).sub2[Entity],
                    Entity(
                        Entity {
                            name: "sub2",
                            attributes: [
                                EntityAttribute {
                                    name: Reference(
                                        "y2",
                                    ),
                                    ty: Simple(
                                        Real,
                                    ),
                                    optional: false,
                                },
                            ],
                            constraint: None,
                            subtype_of: Some(
                                SubTypeDecl {
                                    entity_references: [
                                        "base",
                                    ],
                                },
                            ),
                            derive_clause: None,
                            inverse_clause: None,
                            unique_clause: None,
                            where_clause: None,
                        },
                    ),
                ),
            ],
        }
        "###);
    }
}
