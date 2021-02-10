//! Tokenize EXPRESS language into [syntax_tree::SyntaxTree]
//!
//! This submodule responsible for tokenize of EXPRESS language input into a [syntax_tree::SyntaxTree] struct.
//! Following steps of compile, i.e. semantics analysis and Rust code generation will be handled by
//! other submodules.
//!
//! This submodule is based on [nom](https://github.com/Geal/nom) parser combinater.
//!
//! Example
//! --------
//!
//! EXPRESS Language string is parsed into [syntax_tree::SyntaxTree]:
//!
//! ```
//! let schemas = espr::parser::syntax_tree::SyntaxTree::parse(r#"
//! SCHEMA one;
//!   ENTITY first;
//!     m_ref : second;
//!     fattr : STRING;
//!   END_ENTITY;
//!   ENTITY second;
//!     sattr : STRING;
//!   END_ENTITY;
//! END_SCHEMA;
//!
//! SCHEMA geometry0;
//!   ENTITY point;
//!     x, y, z: REAL;
//!   END_ENTITY;
//! END_SCHEMA;
//! "#.trim()).unwrap();
//! ```

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
mod syntax_tree;
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
pub use syntax_tree::*;
pub use types::*;
