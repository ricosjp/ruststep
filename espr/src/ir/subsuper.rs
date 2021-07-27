//! Subtype-Supertype Graph

use super::*;
use std::collections::HashMap;

#[derive(Debug)]
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
                let entity_scope = scope.pushed(ScopeType::Entity, &entity.name);
                // Be sure that `ENTITY A SUBTYPE OF (B)` means `B` is *supertype* of `A`
                if let Some(supertypes) = &entity.subtype_of {
                    for name in &supertypes.entity_references {
                        let sub = Path::new(&entity_scope, ScopeType::Entity, &entity.name);
                        let sup = ns.resolve(&entity_scope, name)?;
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

    pub fn get_supertypes(&self, name: &str, scope: &Scope) -> Result<&[Path], SemanticError> {
        let sub = Path::new(scope, ScopeType::Entity, name);
        self.sub_to_super
            .get(&sub)
            .map(|ty| ty.as_slice())
            .ok_or(SemanticError::TypeNotFound {
                name: name.to_string(),
                scope: scope.clone(),
            })
    }

    pub fn get_subtypes(&self, name: &str, scope: &Scope) -> Result<&[Path], SemanticError> {
        let sup = Path::new(scope, ScopeType::Entity, name);
        self.super_to_sub
            .get(&sup)
            .map(|ty| ty.as_slice())
            .ok_or(SemanticError::TypeNotFound {
                name: name.to_string(),
                scope: scope.clone(),
            })
    }
}
