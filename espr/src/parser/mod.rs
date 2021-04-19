//! Tokenize EXPRESS language into [SyntaxTree]
//!
//! This submodule responsible for tokenize of EXPRESS language input into a [SyntaxTree] struct.
//! Following steps of compile, i.e. semantics analysis and Rust code generation will be handled by
//! other submodules.
//!
//! This submodule is based on [nom](https://github.com/Geal/nom) parser combinater.

#[cfg(doc)]
use crate::ast::*;

pub mod combinator;

mod basis;
mod entity;
mod expression;
mod identifier;
mod literal;
mod remark;
mod schema;
mod stmt;
mod subsuper;
mod types;

pub use basis::*;
pub use entity::*;
pub use expression::*;
pub use identifier::*;
pub use literal::*;
pub use remark::*;
pub use schema::*;
pub use stmt::*;
pub use subsuper::*;
pub use types::*;
