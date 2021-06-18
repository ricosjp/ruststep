use itertools::*;
use std::{cmp, fmt};

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

/// Scope declaration
///
/// Partial Order
/// --------------
/// Scope is partially ordered in terms of the sub-scope relation:
///
/// ```
/// # use espr::semantics::*;
/// let root = Scope::root();
/// let schema = root.pushed(ScopeType::Schema, "schema");
/// assert!(root > schema); // schema scope is sub-scope of root scope
/// ```
///
/// Be sure that this is not total order:
///
/// ```
/// # use espr::semantics::*;
/// let root = Scope::root();
/// let schema1 = root.pushed(ScopeType::Schema, "schema1");
/// let schema2 = root.pushed(ScopeType::Schema, "schema2");
///
/// // schema1 and schema2 are both sub-scope of root,
/// assert!(root > schema1);
/// assert!(root > schema2);
///
/// // but they are independent. Comparison always returns false:
/// assert!(!(schema1 <= schema2));
/// assert!(!(schema1 >= schema2));
/// ```
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

impl PartialOrd for Scope {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        for (lhs, rhs) in self.0.iter().zip(other.0.iter()) {
            if lhs != rhs {
                return None;
            }
        }
        if self.0.len() == other.0.len() {
            Some(cmp::Ordering::Equal)
        } else if self.0.len() > other.0.len() {
            Some(cmp::Ordering::Less)
        } else {
            Some(cmp::Ordering::Greater)
        }
    }
}

impl Scope {
    pub fn root() -> Self {
        Self(Vec::new())
    }

    pub fn pushed(&self, ty: ScopeType, name: &str) -> Self {
        let mut new = self.clone();
        new.0.push((ty, name.to_string()));
        new
    }

    /// Pop the last scope
    ///
    /// Returns `None` when `self` is root.
    pub fn popped(&self) -> Option<Self> {
        let mut new = self.clone();
        let _current = new.0.pop()?;
        Some(new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope() {
        let root = Scope::root();
        assert_eq!(format!("{}", root), "");

        let schema = root.pushed(ScopeType::Schema, "schema1");
        assert_eq!(format!("{}", schema), "schema1");
        assert_eq!(format!("{:?}", schema), "Scope(schema1[Schema])");

        let entity = schema.pushed(ScopeType::Entity, "entity1");
        assert_eq!(format!("{}", entity), "schema1.entity1");
        assert_eq!(
            format!("{:?}", entity),
            "Scope(schema1[Schema].entity1[Entity])"
        );
    }
}
