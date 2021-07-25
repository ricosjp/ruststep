//! Subtype-Supertype Graph

use super::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SubSuperGraph {
    pub super_to_sub: HashMap<TypeRef /* super */, Vec<TypeRef /* sub */>>,
}

impl SubSuperGraph {
    pub fn new(ns: &Namespace, st: &SyntaxTree) -> Result<Self, SemanticError> {
        let super_to_sub = HashMap::new();
        let root = Scope::root();
        for schema in &st.schemas {
            let scope = root.pushed(ScopeType::Schema, &schema.name);
            for entity in &schema.entities {
                let entity_scope = scope.pushed(ScopeType::Entity, &entity.name);
                if let Some(subtypes) = &entity.subtype {
                    for name in &subtypes.entity_references {
                        let ty = ns.lookup_type(&entity_scope, name)?;
                        dbg!(ty);
                    }
                }
            }
        }
        Ok(SubSuperGraph { super_to_sub })
    }
}
