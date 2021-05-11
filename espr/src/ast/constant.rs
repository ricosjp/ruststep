use crate::ast::{expression::*, types::*};

#[derive(Debug, Clone, PartialEq)]
pub struct Constant {
    pub bodies: Vec<ConstantBody>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantBody {
    pub name: String,
    pub ty: UnderlyingType,
    pub expression: Expression
}