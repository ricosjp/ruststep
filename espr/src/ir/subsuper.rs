//! Subtype-Supertype Graph

use super::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SubSuperGraph {
    pub super_to_sub: HashMap<Path, Vec<Path>>,
}

impl SubSuperGraph {
    pub fn new(ns: &Namespace, st: &SyntaxTree) -> Result<Self, SemanticError> {
        todo!()
    }
}
