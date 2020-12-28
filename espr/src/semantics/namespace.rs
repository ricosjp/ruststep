use crate::{error::*, parser::*, semantics::*};
use maplit::hashmap;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IdentifierType {
    Entity,
    Schema,
    Attribute,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Namespace(HashMap<Scope, HashMap<IdentifierType, Vec<String>>>);

impl Namespace {
    pub fn new(syn: &SyntaxTree) -> Result<Self, Error> {
        let mut names = HashMap::new();
        let mut current_scope = Scope::root();
        names.insert(
            current_scope.clone(),
            hashmap! {
                IdentifierType::Schema => syn.schemas.iter().map(|schema| schema.name.clone()).collect()
            },
        );

        for schema in &syn.schemas {
            current_scope = current_scope.pushed(ScopeType::Schema, &schema.name);
            names.insert(
                current_scope.clone(),
                hashmap! {
                    IdentifierType::Entity => schema.entities.iter().map(|e| e.name.clone()).collect()
                },
            );

            for entity in &schema.entities {
                current_scope = current_scope.pushed(ScopeType::Entity, &entity.name);
                let attrs = entity
                    .attributes
                    .iter()
                    .map(|(name, _ty)| name.clone())
                    .collect();
                names.insert(
                    current_scope.clone(),
                    hashmap! {
                        IdentifierType::Attribute => attrs
                    },
                );
                current_scope = current_scope.popd().expect("Never be root");
            }
            current_scope = current_scope.popd().expect("Never be root");
        }
        Ok(Self(names))
    }

    pub fn lookup_type(&self, _current_scope: &Scope, _name: &str) -> Result<Type, SemanticError> {
        todo!()
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
