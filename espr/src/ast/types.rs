//! AST for type declaration

use crate::ast::{algorithm::*, expression::*};
use derive_more::From;

#[cfg(doc)]
use crate::parser::*;

/// `EXTENSIBLE` and `GENERIC_ENTITY` keywords for [select_type] and [enumeration_type]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Extensiblity {
    /// No `EXTENSIBLE`
    None,
    /// `EXTENSIBLE`
    Extensible,
    /// `EXTENSIBLE GENERIC_ENTITY`, which is allowed only for `SELECT`
    GenericEntity,
}

/// Output of [type_decl]
#[derive(Debug, Clone, PartialEq)]
pub struct TypeDecl {
    pub type_id: String,
    pub underlying_type: UnderlyingType,
    pub where_clause: Option<WhereClause>,
}

/// Output of [constructed_types]
#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum ConstructedType {
    Enumeration(EnumerationType),
    Select(SelectType),
}

/// Output of [underlying_type]
#[derive(Debug, Clone, PartialEq, From)]
pub enum UnderlyingType {
    Constructed(ConstructedType),
    Concrete(ConcreteType),
}

/// Output of [select_type]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectType {
    pub extensiblity: Extensiblity,
    pub types: Vec<String>,
}

/// Output of [width_spec]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WidthSpec {
    pub width: usize,
    pub fixed: bool,
}

/// Output of [simple_types]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimpleType {
    /// 8.1.1 Number data type
    Number,
    /// 8.1.2 Real data type
    Real,
    /// 8.1.3 Integer data type
    Integer,
    /// 8.1.4 Logical data type
    Logical,
    /// 8.1.5 Boolen data type
    Boolen,
    /// 8.1.6 String data type
    String_ { width_spec: Option<WidthSpec> },
    /// 8.1.7 Binary data type
    Binary { width_spec: Option<WidthSpec> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParameterType {
    Named(String),
    Simple(SimpleType),
    Set {
        ty: Box<ParameterType>,
        bound_spec: Option<Bound>,
    },
    Bag {
        ty: Box<ParameterType>,
        bound_spec: Option<Bound>,
    },
    List {
        ty: Box<ParameterType>,
        bound_spec: Option<Bound>,
        unique: bool,
    },
    Array {
        ty: Box<ParameterType>,
        bound_spec: Option<Bound>,
        unique: bool,
        optional: bool,
    },
    Aggregate {
        ty: Box<ParameterType>,
        label: Option<String>,
    },
    GenericEntity(Option<String>),
    Generic(Option<String>),
}

/// Output of [enumeration_type]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumerationType {
    pub extensiblity: Extensiblity,
    pub items: Vec<String>,
}

/// Output of [concrete_types]
#[derive(Debug, Clone, PartialEq)]
pub enum ConcreteType {
    Simple(SimpleType),
    Reference(String),
    Set {
        bound: Option<Bound>,
        base: Box<ConcreteType>,
    },
    Bag {
        bound: Option<Bound>,
        base: Box<ConcreteType>,
    },
    List {
        unique: bool,
        bound: Option<Bound>,
        base: Box<ConcreteType>,
    },
    Array {
        unique: bool,
        optional: bool,
        bound: Bound,
        base: Box<ConcreteType>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bound {
    pub lower: Expression,
    pub upper: Expression,
}
