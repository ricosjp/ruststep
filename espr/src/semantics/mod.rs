//! Legalize [SyntaxTree] into [IR]

mod namespace;
mod schema;
mod scope;
mod types;

pub use namespace::*;
pub use schema::*;
pub use scope::*;
pub use types::*;

use crate::parser::SyntaxTree;

/// Intermediate Representation
pub struct IR {}

impl IR {
    pub fn legalize(_syn: SyntaxTree) -> Result<Self, ()> {
        todo!()
    }
}
