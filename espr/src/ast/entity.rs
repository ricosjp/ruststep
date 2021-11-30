//! AST for entity declarations

use crate::{
    ast::{algorithm::*, error::*, expression::*, types::*},
    parser::*,
};
use std::str::FromStr;

/// Parsed result of EXPRESS's ENTITY
#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    /// Name of this entity type
    pub name: String,

    /// attribute name and types
    ///
    /// Be sure that this "type" is a string, not validated type in this timing
    pub attributes: Vec<EntityAttribute>,

    pub constraint: Option<Constraint>,
    pub subtype_of: Option<SubTypeDecl>,

    pub derive_clause: Option<DeriveClause>,
    pub inverse_clause: Option<InverseClause>,
    pub unique_clause: Option<UniqueClause>,
    pub where_clause: Option<WhereClause>,
}

impl FromStr for Entity {
    type Err = TokenizeFailed;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use nom::Finish;
        let input = input.trim();
        let (_input, (record, _remarks)) = entity_decl(input)
            .finish()
            .map_err(|err| TokenizeFailed::new(input, err))?;
        Ok(record)
    }
}

impl Entity {
    pub fn has_supertype_decl(&self) -> bool {
        if let Some(c) = &self.constraint {
            match c {
                Constraint::AbstractSuperType(..) | Constraint::SuperTypeRule(..) => true,
                Constraint::AbstractEntity => false,
            }
        } else {
            false
        }
    }
}

/// Intermediate output of [entity_body]
#[derive(Debug, Clone, PartialEq)]
pub struct EntityBody {
    pub attributes: Vec<EntityAttribute>,
    pub derive_clause: Option<DeriveClause>,
    pub inverse_clause: Option<InverseClause>,
    pub unique_clause: Option<UniqueClause>,
    pub where_clause: Option<WhereClause>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeDecl {
    Reference(String),
    Qualified {
        /// Like `\point`
        group: String,
        /// Like `.x`
        attribute: String,
        /// For [redeclared_attribute]
        rename: Option<String>,
    },
}

// for easy testing
impl<'a> PartialEq<&'a str> for AttributeDecl {
    fn eq(&self, other: &&'a str) -> bool {
        match self {
            AttributeDecl::Reference(name) => other.eq(name),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityAttribute {
    pub name: AttributeDecl,
    pub ty: Type,
    pub optional: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeriveClause {
    pub attributes: Vec<DerivedAttribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DerivedAttribute {
    pub attr: AttributeDecl,
    pub ty: Type,
    pub expr: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InverseClause {
    pub attributes: Vec<InverseAttribute>,
}

/// Attribute of an inverse clause parsed by [inverse_attr]
///
/// From ISO 10303-11 document,
///
/// ```text
/// ENTITY door;
///   handle : knob;                -- inverse relationship for this attribute
///   hinges : SET [1:?] OF hinge;
/// END_ENTITY;
///
/// ENTITY knob;
/// ...
/// INVERSE
///   opens : door FOR handle;
/// (* ^      ^        ^
///    |      |        attribute name used in the parent entity
///    |      The entity which has `SELF` as attribute
///    name of this inverse relationship *)
/// END_ENTITY;
/// ```
///
/// This means
///
/// > knobs can only exist if they are used in the role of handle in one instance of a door
///
#[derive(Debug, Clone, PartialEq)]
pub struct InverseAttribute {
    /// Name of this inverse relationship
    ///
    /// `opens` in above example
    pub name: AttributeDecl,

    /// The entity name which has `SELF` as an attribute
    ///
    /// `door` in above example
    pub dest: String,

    /// Used if `SET` or `BAG` for parent entity specification
    pub dest_aggregation: AggregationOption,

    /// The attribute name used in the parent entity
    ///
    /// `handle` in above example
    pub attribute: String,

    /// Prefix of the attribute, used if the attribute is a sub-attribute of `dest` entity
    pub attribute_prefix: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AggregationOption {
    Set { bound: Option<Bound> },
    Bag { bound: Option<Bound> },
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constraint {
    AbstractEntity,
    AbstractSuperType(Option<SuperTypeExpression>),
    SuperTypeRule(SuperTypeExpression),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubTypeDecl {
    pub entity_references: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SuperTypeExpression {
    Reference(String),
    AndOr { factors: Vec<SuperTypeExpression> },
    And { terms: Vec<SuperTypeExpression> },
    OneOf { exprs: Vec<SuperTypeExpression> },
}

impl SuperTypeExpression {
    /// Get the list of subtype names
    ///
    /// - When `ENTITY A SUPERTYPE OF (B)`, it means `A` is supertype of `B`, i.e. `B` is subtype of `A`.
    ///   Thus, this should be `as_subtype_names` rather than `as_supertype_names`
    ///   because it returns `["B"]` for this case and they are subtypes of `self`
    /// - This ignores the differences between `ONE_OF`, `ANDOR`, and `AND`
    pub fn as_subtype_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = Vec::new();
        match self {
            SuperTypeExpression::Reference(name) => names.push(name),
            SuperTypeExpression::OneOf { exprs }
            | SuperTypeExpression::AndOr { factors: exprs }
            | SuperTypeExpression::And { terms: exprs } => {
                for expr in exprs {
                    let mut sub_names = expr.as_subtype_names();
                    names.append(&mut sub_names);
                }
            }
        }
        names
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubTypeConstraint {
    pub name: String,
    pub entity: String,
    pub is_abstract: bool,
    pub total_over: Option<Vec<String>>,
    pub expr: Option<SuperTypeExpression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UniqueClause {
    pub rules: Vec<UniqueRule>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UniqueRule {
    pub name: Option<String>,
    pub attributes: Vec<AttributeDecl>,
}

#[cfg(test)]
mod tests {
    use crate::parser;

    #[test]
    fn as_subtype_names() {
        let (_, (expr, _)) = parser::supertype_expression("employee ANDOR sutudent").unwrap();
        assert_eq!(expr.as_subtype_names(), &["employee", "sutudent"]);

        let (_, (expr, _)) =
            parser::supertype_expression("ONEOF(male,female) AND ONEOF(citizen,alien)").unwrap();
        assert_eq!(
            expr.as_subtype_names(),
            &["male", "female", "citizen", "alien"]
        );

        let (_, (expr, _)) = parser::supertype_expression("a ANDOR b AND c").unwrap();
        assert_eq!(expr.as_subtype_names(), &["a", "b", "c"]);

        let (_, (expr, _)) = parser::supertype_expression("(a ANDOR b) AND c").unwrap();
        assert_eq!(expr.as_subtype_names(), &["a", "b", "c"]);
    }
}
