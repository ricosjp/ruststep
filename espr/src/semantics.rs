use crate::{error::*, parser::*};
use std::{collections::HashMap, fmt};

/// Identifier in EXPRESS language must be one of scopes described in
/// "Table 9 – Scope and identifier defining items"
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Scope {
    /// Root of all scopes
    Global,
    Entity {
        parent: Box<Scope>,
        name: String,
    },
    Alias {
        parent: Box<Scope>,
        name: String,
    },
    Function {
        parent: Box<Scope>,
        name: String,
    },
    Procedure {
        parent: Box<Scope>,
        name: String,
    },
    Query {
        parent: Box<Scope>,
        name: String,
    },
    Repeat {
        parent: Box<Scope>,
        name: String,
    },
    Rule {
        parent: Box<Scope>,
        name: String,
    },
    Schema {
        parent: Box<Scope>,
        name: String,
    },
    SubType {
        parent: Box<Scope>,
        name: String,
    },
    Type {
        parent: Box<Scope>,
        name: String,
    },
}

impl Scope {
    pub fn name(&self) -> &str {
        use Scope::*;
        match self {
            Global => "",
            Entity { name, .. }
            | Alias { name, .. }
            | Function { name, .. }
            | Procedure { name, .. }
            | Query { name, .. }
            | Repeat { name, .. }
            | Rule { name, .. }
            | Schema { name, .. }
            | SubType { name, .. }
            | Type { name, .. } => &name,
        }
    }

    pub fn parent(&self) -> Option<Scope> {
        use Scope::*;
        match self {
            Global => None,
            Entity { parent, .. }
            | Alias { parent, .. }
            | Function { parent, .. }
            | Procedure { parent, .. }
            | Query { parent, .. }
            | Repeat { parent, .. }
            | Rule { parent, .. }
            | Schema { parent, .. }
            | SubType { parent, .. }
            | Type { parent, .. } => Some(*parent.clone()),
        }
    }

    /// ```
    /// use espr::semantics::*;
    ///
    /// let root = Scope::Global;
    /// assert_eq!(root.full_name(), "");
    ///
    /// let schema = Scope::Schema {
    ///   parent: Box::new(root),
    ///   name: "my_schema".to_string()
    /// };
    /// assert_eq!(schema.full_name(), "my_schema");
    ///
    /// let f = Scope::Function {
    ///   parent: Box::new(schema),
    ///   name: "func1".to_string(),
    /// };
    /// assert_eq!(f.full_name(), "my_schema.func1");
    /// ```
    pub fn full_name(&self) -> String {
        if let Some(parent) = self.parent() {
            format!("{}.{}", parent.name(), self.name())
        } else {
            "".to_string()
        }
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

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
