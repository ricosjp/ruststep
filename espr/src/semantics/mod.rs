//! Legalize [SyntaxTree] into [IR]

mod namespace;
mod scope;

pub use namespace::*;
pub use scope::*;

use crate::parser::SyntaxTree;

/// Intermediate Representation
pub struct IR {}

impl IR {
    pub fn legalize(_syn: SyntaxTree) -> Result<Self, ()> {
        todo!()
    }
}
