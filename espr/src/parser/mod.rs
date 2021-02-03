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

pub mod basis;
pub mod entity;
pub mod expression;
pub mod identifier;
pub mod literal;
pub mod remark;
pub mod schema;
pub mod subsuper;
pub mod syntax_tree;
pub mod types;
pub mod util;
