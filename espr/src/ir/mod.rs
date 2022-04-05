//! Intermediate Representation ([IR]) legalized (semantically analyzed) from [SyntaxTree]

mod complex_entity;
mod constraints;
mod entity;
mod namespace;
mod schema;
mod scope;
mod type_decl;
mod type_ref;

pub use complex_entity::*;
pub use constraints::*;
pub use entity::*;
pub use namespace::*;
pub use schema::*;
pub use scope::*;
pub use type_decl::*;
pub use type_ref::*;

use crate::ast::SyntaxTree;
use thiserror::Error;

/// Semantic errors
#[derive(Debug, Error)]
pub enum SemanticError {
    #[error("Not found the Type {name} referred in scope {scope}")]
    TypeNotFound { name: String, scope: Scope },

    #[error("Invalid path: {0}")]
    InvalidPath(Path),

    #[error("Same item ({0}) is declared multiple times")]
    DuplicatedDeclaration(Path),
}

/// Legalize partial parsed input into corresponding intermediate representation
pub trait Legalize: Sized {
    type Input;
    fn legalize(
        namespace: &Namespace,
        sub_super_graph: &Constraints,
        scope: &Scope,
        syn: &Self::Input,
    ) -> Result<Self, SemanticError>;
}

/// Intermediate Representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IR {
    pub schemas: Vec<Schema>,
}

impl IR {
    pub fn from_syntax_tree(st: &SyntaxTree) -> Result<Self, SemanticError> {
        let ns = Namespace::new(st);
        let ss = Constraints::new(&ns, st)?;
        let ir = Self::legalize(&ns, &ss, &Scope::root(), st)?;
        Ok(ir)
    }
}

impl Legalize for IR {
    type Input = SyntaxTree;
    fn legalize(
        ns: &Namespace,
        ss: &Constraints,
        scope: &Scope,
        syn: &SyntaxTree,
    ) -> Result<Self, SemanticError> {
        let schemas = syn
            .schemas
            .iter()
            .map(|schema| Schema::legalize(ns, ss, scope, schema))
            .collect::<Result<Vec<Schema>, SemanticError>>()?;
        Ok(IR { schemas })
    }
}
