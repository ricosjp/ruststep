use crate::ast::{algorithm::*, entity::*, types::*};

/// Parsed result of EXPRESS's SCHEMA
#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    pub name: String,
    pub entities: Vec<Entity>,
    pub types: Vec<TypeDecl>,
    pub functions: Vec<Function>,
    pub procedures: Vec<Procedure>,
    pub rules: Vec<Rule>,
    pub constants: Vec<Constant>,
    pub interfaces: Vec<InterfaceSpec>,
    pub subtype_constraints: Vec<SubTypeConstraint>,
}
