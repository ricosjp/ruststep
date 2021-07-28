//! AST for type declaration

use crate::ast::{algorithm::*, expression::*};

#[cfg(doc)]
use crate::parser::*;

/// Type declaration by [type_decl].
#[derive(Debug, Clone, PartialEq)]
pub struct TypeDecl {
    pub type_id: String,
    pub underlying_type: ParameterType,
    pub where_clause: Option<WhereClause>,
}

/// Parameter type appears when *using* the type
/// e.g. in attribute definition, function parameter, and so on.
#[derive(Debug, Clone, PartialEq)]
pub enum ParameterType {
    Simple(SimpleType),
    Named(String),
    Set {
        base: Box<ParameterType>,
        bound: Option<Bound>,
    },
    Bag {
        base: Box<ParameterType>,
        bound: Option<Bound>,
    },
    List {
        base: Box<ParameterType>,
        bound: Option<Bound>,
        unique: bool,
    },
    Array {
        base: Box<ParameterType>,
        bound: Option<Bound>,
        unique: bool,
        optional: bool,
    },

    // Constructed Types
    Enumeration {
        extensibility: Extensibility,
        items: Vec<String>,
    },
    Select {
        extensibility: Extensibility,
        types: Vec<String>,
    },

    // Parameter Types
    Aggregate {
        base: Box<ParameterType>,
        label: Option<String>,
    },
    GenericEntity(Option<String>),
    Generic(Option<String>),
}

/// Primitive types parsed by [simple_types]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// Output of [width_spec]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WidthSpec {
    pub width: usize,
    pub fixed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bound {
    pub lower: Expression,
    pub upper: Expression,
}

/// `EXTENSIBLE` and `GENERIC_ENTITY` keywords for [select_type] and [enumeration_type]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Extensibility {
    /// No `EXTENSIBLE`
    None,
    /// `EXTENSIBLE`
    Extensible,
    /// `EXTENSIBLE GENERIC_ENTITY`, which is allowed only for `SELECT`
    GenericEntity,
}
