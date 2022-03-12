//! Partial complex entities described in ISO-10303-11 Annex B

use super::*;
use crate::ast;
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
        let mut instantiables = Vec::new();
        let root = Scope::root();
        for schema in &st.schemas {
            let scope = root.schema(&schema.name);

            // Be sure that `SUPERTYPE OF` declaration with complex constraint
            // using `ONEOF`, `AND` and `ANDOR` are deprecated:
            //
            // ISO-10303-11 (2004, en) Page 56, Note 1
            // > In order that existing schemas remain valid,
            // > the declaration of subtype/supertype constraints
            // > that use the keywords oneof, andor, or and within
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
                        instantiables.push((path, Instantiables::from_expr(ns, &scope, expr)?));
                    }
                    _ => continue,
                }
            }
            // TODO: SUBTYPE_CONSTRAINTS
        }

        // TODO Add implicit constraints

        // Replace indices to Path using Namespace
        let instantiables = instantiables
            .into_iter()
            .map(|(path, it)| {
                let it: Vec<Vec<Path>> = it
                    .parts
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
                    .collect();
                (path, it)
            })
            .collect();
        Ok(Constraints { instantiables })
    }

    pub fn is_supertype(&self, path: &Path) -> bool {
        self.instantiables.contains_key(path)
    }
}
