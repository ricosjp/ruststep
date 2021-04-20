//! Parser combinator for EXPRESS language
//!
//! This submodule responsible for tokenize of EXPRESS language input into a [SyntaxTree] struct,
//! and provides parser combinators based on [nom](https://github.com/Geal/nom).
//!
//! Most parsers correspond to EXPRESS language grammers defined in [ISO-10303-11].
//! Each documentation of the parsers contains wirth syntax notation (WSN).
//! For example,
//!
//! ```text
//! 124 digit = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 .
//! ```
//!
//! [digit] matches to a digit character.
//! The head number, `124`, is the serial number of this WSN in [ISO-10303-11].
//!
//! [ISO-10303-11]: https://www.iso.org/standard/38047.html
//!
//! Remarks
//! --------
//!
//! EXPRESS language has two forms of "remarks" which corresponds to "comments" in Rust
//!
//! ```text
//! (* this is called "embedded remark" *)
//! -- this is called "tail remark"
//! ```
//!
//! Each of remarks are handled by [embedded_remark] and [tail_remark].
//! For handling remarks appear in arbitrary position,
//! we re-define nom's combinators in [combinator] to stack remarks onto a `Vec<Remark>` in appeared order.
//!

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
