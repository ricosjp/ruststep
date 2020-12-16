use crate::{error::*, parser::*};
use std::collections::HashMap;

/// Additional names
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Entity { attribute_names: Vec<String> },
}

/// Identifiers in schemas
#[derive(Debug, Clone, PartialEq)]
pub struct Namespace(HashMap<String, HashMap<String, Type>>);

impl Namespace {
    pub fn new(schemas: &[Schema]) -> Result<Self, crate::error::Error> {
        let mut names = HashMap::new();
        for schema in schemas {
            let schema_name = schema.name.clone();
            let mut local_names = HashMap::new();
            for entity in &schema.entities {
                let attribute_names = entity
                    .attributes
                    .iter()
                    .map(|(name, _ty)| name.clone())
                    .collect();
                if local_names
                    .insert(entity.name.clone(), Type::Entity { attribute_names })
                    .is_some()
                {
                    return Err(Error::DuplicatedEntity {
                        schema: schema_name,
                        name: entity.name.clone(),
                    });
                }
            }
            if names.insert(schema_name.clone(), local_names).is_some() {
                return Err(Error::DuplicatedSchema { name: schema_name });
            }
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
