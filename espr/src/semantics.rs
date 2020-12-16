use crate::parser::*;
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
    pub fn new(schemas: &[Schema]) -> Self {
        let mut names = HashMap::new();
        for schema in schemas {
            let mut local_names = HashMap::new();
            for entity in &schema.entities {
                let attribute_names = entity
                    .attributes
                    .iter()
                    .map(|(name, _ty)| name.clone())
                    .collect();
                local_names
                    .insert(entity.name.clone(), Type::Entity { attribute_names })
                    .expect("entity name is duplicated");
            }
            names
                .insert(schema.name.clone(), local_names)
                .expect("Schema name is duplicated");
        }
        Self(names)
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
