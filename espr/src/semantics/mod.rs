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

/// Intermediate Representation
pub struct IR {}

impl IR {
    pub fn legalize(_syn: &SyntaxTree) -> Result<Self, ()> {
        todo!()
    }
}

impl ToTokens for IR {
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        todo!()
    }
}
