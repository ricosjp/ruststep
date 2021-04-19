use crate::ast::{algorithm::*, expression::*, types::*};

#[cfg(doc)]
use crate::parser::*;

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
    pub subtype: Option<SubTypeDecl>,

    pub derive_clause: Option<DeriveClause>,
    pub inverse_clause: Option<InverseClause>,
    pub unique_clause: Option<UniqueClause>,
    pub where_clause: Option<WhereClause>,
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
        group: Option<String>,
        /// Like `.x`
        attribute: Option<String>,
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
    pub ty: ParameterType,
    pub optional: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeriveClause {
    pub attributes: Vec<DerivedAttribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DerivedAttribute {
    pub attr: AttributeDecl,
    pub ty: ParameterType,
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
