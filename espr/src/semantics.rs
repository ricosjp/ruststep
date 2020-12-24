use crate::{error::*, parser::*};
use itertools::*;
use maplit::hashmap;
use std::{collections::HashMap, fmt};

/// Identifier in EXPRESS language must be one of scopes described in
/// "Table 9 – Scope and identifier defining items"
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScopeType {
    Entity,
    Alias,
    Function,
    Procedure,
    Query,
    Repeat,
    Rule,
    Schema,
    SubType,
    Type,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Scope(Vec<(ScopeType, String)>);

// Custom debug output like: `schema.entity`
impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.iter().map(|(_ty, name)| name).join("."))
    }
}

// Custom debug output like: `Scope(schema[Schema].entity[Entity])`
impl fmt::Debug for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Scope(")?;
        for (i, (ty, name)) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, ".")?;
            }
            write!(f, "{}[{:?}]", name, ty)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl Scope {
    fn root() -> Self {
        Self(Vec::new())
    }

    fn pushed(&self, ty: ScopeType, name: &str) -> Self {
        let mut new = self.clone();
        new.0.push((ty, name.to_string()));
        new
    }

    fn popd(&self) -> Self {
        let mut new = self.clone();
        new.0.pop().expect("Cannot get the parent of root scope");
        new
    }
}

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
    fn scope() {
        let root = Scope::root();
        assert_eq!(format!("{}", root), "");

        let schema1 = Scope(vec![(ScopeType::Schema, "schema1".to_string())]);
        assert_eq!(format!("{}", schema1), "schema1");
        assert_eq!(format!("{:?}", schema1), "Scope(schema1[Schema])");

        let entity = Scope(vec![
            (ScopeType::Schema, "schema1".to_string()),
            (ScopeType::Entity, "entity1".to_string()),
        ]);
        assert_eq!(format!("{}", entity), "schema1.entity1");
        assert_eq!(
            format!("{:?}", entity),
            "Scope(schema1[Schema].entity1[Entity])"
        );
    }

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
