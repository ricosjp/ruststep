//! Legalize [SyntaxTree] into [IR]

mod entity;
mod namespace;
mod schema;
mod scope;
mod types;

pub use entity::*;
pub use namespace::*;
pub use schema::*;
pub use scope::*;
pub use types::*;

use crate::parser::SyntaxTree;
use proc_macro2::TokenStream;
use quote::*;

/// Semantic errors
#[derive(Debug)]
pub enum SemanticError {}

/// Legalize partial parsed input into corresponding intermediate representation
pub trait Legalize: Sized {
    type Input;
    fn legalize(namespace: &Namespace, syn: &Self::Input) -> Result<Self, SemanticError>;
}

/// Intermediate Representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IR {
    pub schemas: Vec<Schema>,
}

impl Legalize for IR {
    type Input = SyntaxTree;
    fn legalize(_ns: &Namespace, _syn: &SyntaxTree) -> Result<Self, SemanticError> {
        todo!()
    }
}

impl ToTokens for IR {
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        todo!()
    }
}
