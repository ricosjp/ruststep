//! Subtype-Supertype Graph

use super::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SubSuperGraph {
    pub super_to_sub: HashMap<TypeRef /* super */, Vec<TypeRef /* sub */>>,
}

impl SubSuperGraph {
    pub fn new(ns: &Namespace, st: &SyntaxTree) -> Result<Self, SemanticError> {
        let mut super_to_sub = HashMap::new();
        let root = Scope::root();
        for schema in &st.schemas {
            let scope = root.pushed(ScopeType::Schema, &schema.name);
            for entity in &schema.entities {
                let entity_scope = scope.pushed(ScopeType::Entity, &entity.name);
                let sub = TypeRef::Named {
                    name: entity.name.clone(),
                    scope: entity_scope.clone(),
                    is_simple: false,
                };
                if let Some(supertypes) = &entity.subtype_of {
                    for name in &supertypes.entity_references {
                        let sup = ns.lookup_type(&entity_scope, name)?;
                        let subs: &mut Vec<_> = super_to_sub.entry(sup).or_default();
                        subs.push(sub.clone());
                    }
                }
            }
        }
        Ok(SubSuperGraph { super_to_sub })
    }
}
