//! Partial complex entities described in ISO-10303-11 Annex B

use super::*;
use crate::ast;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

/// Global constraints in EXPRESS components
#[derive(Debug, PartialEq, Eq)]
pub struct Constraints {
    /// Each super-type can be instantiable as its subtypes,
    /// but possible subtypes cannot be determined from local description in EXPRESS.
    pub instantiables: HashMap<Path, Vec<Vec<Path>>>,
}

impl Constraints {
    pub fn new(ns: &Namespace, st: &SyntaxTree) -> Result<Self, SemanticError> {
        let mut instantiables = HashMap::new();
        let root = Scope::root();
        for schema in &st.schemas {
            let scope = root.schema(&schema.name);

            // Be sure that `SUPERTYPE OF` declaration with complex constraint
            // using `ONEOF`, `AND` and `ANDOR` are deprecated:
            //
            // ISO-10303-11 (2004, en) Page 56, Note 1
            // > In order that existing schemas remain valid,
            // > the declaration of subtype/supertype constraints
            // > that use the keywords ONEOF, ANDOR, or AND within
            // > the declaration of an entity, as described in this sub-clause,
            // > remains valid under this edition 2 of EXPRESS.
            // > However, its use is deprecated, and its removal is planned
            // > in future editions. The use of the subtype constraint (see 9.7)
            // > is encouraged instead.
            //
            for entity in &schema.entities {
                match &entity.constraint {
                    Some(ast::Constraint::SuperTypeRule(expr)) => {
                        let path = Path::new(&scope, ScopeType::Entity, &entity.name);
                        instantiables.insert(path, Instantiables::from_expr(ns, &scope, expr)?);
                    }
                    _ => continue,
                }
            }

            for constraint in &schema.subtype_constraints {
                if let Some(expr) = &constraint.expr {
                    let (path, _index) = ns.resolve(&scope, &constraint.entity)?;
                    let is = Instantiables::from_expr(ns, &scope, expr)?;
                    match instantiables.entry(path) {
                        Entry::Occupied(mut e) => {
                            // ISO-10303-11 (2004, en) Annex B.3 "Interpreting the possible complex entity data types"
                            // > Combine the subtype expressions sxi of these constraints
                            // > into a single subtype constraint sti of the form:
                            // > (sx1 ANDOR sx2 ANDOR sx3 . . . ANDOR sxk ).
                            e.insert(Instantiables::andor(vec![e.get().clone(), is]));
                        }
                        Entry::Vacant(e) => {
                            e.insert(is);
                        }
                    }
                }
            }
        }

        // Add default constraints
        //
        // ISO-10303-11 (2004, en) 9.2.5.6 Default constraint between subtypes
        // > If no supertype constraint is mentioned in the declaration of an entity,
        // > the subtypes (if any) shall be mutually inclusive, that is,
        // > as if all subtypes were implicitly mentioned in an ANDOR construct.
        //
        let mut super_to_sub = HashMap::new();
        for schema in &st.schemas {
            let scope = root.schema(&schema.name);
            for entity in &schema.entities {
                let sub = Path::entity(&scope, &entity.name);
                let (_ast, index) = ns.get(&sub)?;
                if let Some(subtype_decl) = &entity.subtype_of {
                    for sup_name in &subtype_decl.entity_references {
                        let (sup, _) = ns.resolve(&scope, sup_name)?;
                        let indices: &mut Vec<usize> = super_to_sub.entry(sup).or_default();
                        indices.push(index);
                    }
                }
            }
        }
        // Apply ANDOR to gathered indices
        for (sup, indices) in super_to_sub {
            let mut is: Vec<_> = indices
                .into_iter()
                .map(|index| Instantiables::single(index))
                .collect();
            match instantiables.entry(sup) {
                Entry::Occupied(mut e) => {
                    is.push(e.get().clone());
                    e.insert(Instantiables::andor(is));
                }
                Entry::Vacant(e) => {
                    e.insert(Instantiables::andor(is));
                }
            }
        }

        // Replace indices to Path using Namespace
        let instantiables = instantiables
            .into_iter()
            .map(|(path, it)| {
                (
                    path,
                    it.parts
                        .iter()
                        .map(|pce| {
                            pce.indices
                                .iter()
                                .map(|index| {
                                    let (path, _ast) = &ns[*index];
                                    path.clone()
                                })
                                .collect()
                        })
                        .collect(),
                )
            })
            .collect();
        Ok(Constraints { instantiables })
    }

    pub fn is_supertype(&self, path: &Path) -> bool {
        self.instantiables.contains_key(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constraint_oneof() {
        let st = ast::SyntaxTree::parse(
            r#"
            SCHEMA test_schema;
              ENTITY pet;
                name : pet_name;
              END_ENTITY;

              SUBTYPE_CONSTRAINT separate_species FOR pet;
                ABSTRACT SUPERTYPE;
                ONEOF(cat, rabbit, dog);
              END_SUBTYPE_CONSTRAINT;

              ENTITY cat SUBTYPE OF (pet);
              END_ENTITY;

              ENTITY rabbit SUBTYPE OF (pet);
              END_ENTITY;

              ENTITY dog SUBTYPE OF (pet);
              END_ENTITY;
            END_SCHEMA;
            "#,
        )
        .unwrap();

        let ns = Namespace::new(&st);
        let c = Constraints::new(&ns, &st).unwrap();

        let scope = Scope::root().schema("test_schema");
        assert_eq!(
            dbg!(c),
            Constraints {
                instantiables: maplit::hashmap! {
                    Path::entity(&scope, "pet") => vec![
                        vec![Path::entity(&scope, "cat")],
                        vec![Path::entity(&scope, "rabbit")],
                        vec![Path::entity(&scope, "dog")],
                    ]
                }
            }
        );
    }

    #[test]
    fn supertype_of_oneof() {
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
        let c = Constraints::new(&ns, &st).unwrap();

        let scope = Scope::root().schema("test_schema");
        assert_eq!(
            dbg!(c),
            Constraints {
                instantiables: maplit::hashmap! {
                    Path::entity(&scope, "base") => vec![
                        vec![Path::entity(&scope, "sub1")],
                        vec![Path::entity(&scope, "sub2")],
                    ]
                }
            }
        );
    }

    #[test]
    fn supertype_of_andor() {
        // Based on `ANDOR` example in ISO-10303-11
        let st = ast::SyntaxTree::parse(
            r#"
            SCHEMA test_schema;
              ENTITY person SUPERTYPE OF (employee ANDOR student);
              END_ENTITY;
              ENTITY employee SUBTYPE OF (person);
              END_ENTITY;
              ENTITY student SUBTYPE OF (person);
              END_ENTITY;
            END_SCHEMA;
            "#,
        )
        .unwrap();

        let ns = Namespace::new(&st);
        let c = Constraints::new(&ns, &st).unwrap();

        let scope = Scope::root().schema("test_schema");
        assert_eq!(
            dbg!(c),
            Constraints {
                instantiables: maplit::hashmap! {
                    Path::entity(&scope, "person") => vec![
                        vec![Path::entity(&scope, "employee")],
                        vec![Path::entity(&scope, "employee"), Path::entity(&scope, "student")],
                        vec![Path::entity(&scope, "student")],
                    ]
                }
            }
        );
    }

    #[test]
    fn supertype_of_and() {
        // Based on `AND` example in ISO-10303-11
        let st = ast::SyntaxTree::parse(
            r#"
            SCHEMA test_schema;
              ENTITY person SUPERTYPE OF (ONEOF(male,female) AND ONEOF(citizen,alien));
              END_ENTITY;
              ENTITY male SUBTYPE OF (person);
              END_ENTITY;
              ENTITY female SUBTYPE OF (person);
              END_ENTITY;
              ENTITY citizen SUBTYPE OF (person);
              END_ENTITY;
              ENTITY alien SUBTYPE OF (person);
              END_ENTITY;
            END_SCHEMA;
            "#,
        )
        .unwrap();

        let ns = Namespace::new(&st);
        let c = Constraints::new(&ns, &st).unwrap();

        let scope = Scope::root().schema("test_schema");
        assert_eq!(
            dbg!(c),
            Constraints {
                instantiables: maplit::hashmap! {
                    Path::entity(&scope, "person") => vec![
                        vec![Path::entity(&scope, "male"), Path::entity(&scope, "citizen")],
                        vec![Path::entity(&scope, "male"), Path::entity(&scope, "alien")],
                        vec![Path::entity(&scope, "female"), Path::entity(&scope, "citizen")],
                        vec![Path::entity(&scope, "female"), Path::entity(&scope, "alien")],
                    ]
                }
            }
        );
    }

    #[test]
    fn default_constraint() {
        let st = ast::SyntaxTree::parse(
            r#"
            SCHEMA test_schema;
              ENTITY person;
              END_ENTITY;
              ENTITY employee SUBTYPE OF (person);
              END_ENTITY;
              ENTITY student SUBTYPE OF (person);
              END_ENTITY;
            END_SCHEMA;
            "#,
        )
        .unwrap();

        let ns = Namespace::new(&st);
        let c = Constraints::new(&ns, &st).unwrap();

        let scope = Scope::root().schema("test_schema");
        assert_eq!(
            dbg!(c),
            Constraints {
                instantiables: maplit::hashmap! {
                    Path::entity(&scope, "person") => vec![
                        vec![Path::entity(&scope, "employee")],
                        vec![Path::entity(&scope, "employee"), Path::entity(&scope, "student")],
                        vec![Path::entity(&scope, "student")],
                    ]
                }
            }
        );
    }
}
