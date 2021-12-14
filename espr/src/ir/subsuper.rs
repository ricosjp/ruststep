//! Subtype-Supertype Graph

use super::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct SubSuperGraph {
    pub super_to_sub: HashMap<Path, Vec<Path>>,
    pub sub_to_super: HashMap<Path, Vec<Path>>,
}

impl SubSuperGraph {
    pub fn new(ns: &Namespace, st: &SyntaxTree) -> Result<Self, SemanticError> {
        let mut super_to_sub = HashMap::new();
        let mut sub_to_super = HashMap::new();
        let root = Scope::root();
        for schema in &st.schemas {
            let scope = root.pushed(ScopeType::Schema, &schema.name);
            // Check `SUBTYPE OF` specification
            for entity in &schema.entities {
                // Be sure that `ENTITY A SUBTYPE OF (B)` means `B` is *supertype* of `A`
                if let Some(supertypes) = &entity.subtype_of {
                    for name in &supertypes.entity_references {
                        let sub = Path::new(&scope, ScopeType::Entity, &entity.name);
                        let sup = ns.resolve(&scope, name)?;
                        let subs: &mut Vec<_> = super_to_sub.entry(sup.clone()).or_default();
                        subs.push(sub.clone());
                        let sups: &mut Vec<_> = sub_to_super.entry(sub).or_default();
                        sups.push(sup);
                    }
                }
            }
        }
        Ok(SubSuperGraph {
            super_to_sub,
            sub_to_super,
        })
    }

    pub fn get_supertypes(&self, sub: &Path) -> Option<&[Path]> {
        self.sub_to_super.get(sub).map(|ty| ty.as_slice())
    }

    pub fn get_subtypes(&self, sup: &Path) -> Option<&[Path]> {
        self.super_to_sub.get(sup).map(|ty| ty.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast, ir};
    use maplit::hashmap;

    //       ┌──────┐
    //       │ base │
    //     ┌─┴──────┴─┐
    //     │          │
    // ┌───▼──┐    ┌──▼───┐
    // │ sub1 │    │ sub2 │
    // └──────┘    └──────┘
    #[test]
    fn subsuper_tree_init() {
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
        dbg!(&st);

        let ns = ir::Namespace::new(&st);
        let ss = ir::SubSuperGraph::new(&ns, &st).unwrap();
        dbg!(&ss);

        let schema_scope = ir::Scope::root().pushed(ir::ScopeType::Schema, "test_schema");
        let base = ir::Path::new(&schema_scope, ir::ScopeType::Entity, "base");
        let sub1 = ir::Path::new(&schema_scope, ir::ScopeType::Entity, "sub1");
        let sub2 = ir::Path::new(&schema_scope, ir::ScopeType::Entity, "sub2");

        assert_eq!(
            &ss.super_to_sub,
            &hashmap! {
                base.clone() => vec![sub1.clone(), sub2.clone()]
            }
        );

        assert_eq!(
            &ss.sub_to_super,
            &hashmap! {
                sub1.clone() => vec![base.clone()],
                sub2.clone() => vec![base.clone()],
            }
        );

        assert_eq!(
            ss.get_subtypes(&base),
            Some([sub1.clone(), sub2.clone()].as_ref())
        );
        assert_eq!(ss.get_subtypes(&sub1), None);
        assert_eq!(ss.get_subtypes(&sub2), None);

        assert_eq!(ss.get_supertypes(&base), None);
        assert_eq!(ss.get_supertypes(&sub1), Some([base.clone()].as_ref()));
        assert_eq!(ss.get_supertypes(&sub2), Some([base.clone()].as_ref()));
    }
}
