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
    pub fn new(schemas: &[Schema]) -> Result<Self, Error> {
        let mut names = HashMap::new();
        let mut current_scope = Scope::root();
        names.insert(
            current_scope.clone(),
            hashmap! {
                IdentifierType::Schema => schemas.iter().map(|schema| schema.name.clone()).collect()
            },
        );

        for schema in schemas {
            // push scope
            current_scope = current_scope.pushed(ScopeType::Schema, &schema.name);
            names.insert(
                current_scope.clone(),
                hashmap! {
                    IdentifierType::Entity => schema.entities.iter().map(|e| e.name.clone()).collect()
                },
            );

            for entity in &schema.entities {
                // push scope
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

                current_scope = current_scope.popd();
            }

            current_scope = current_scope.popd();
        }
        Ok(Self(names))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn namespace() {
        let ns = Namespace::new(&example());
        dbg!(&ns);
    }

    fn example() -> Vec<Schema> {
        crate::parser::parse(
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
        .unwrap()
    }
}
