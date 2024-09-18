//! Generate Rust code using proc-macro utility crates

mod entity;
mod format;
mod schema;
mod simple_type;
mod type_decl;
mod type_ref;

pub use format::rustfmt;
pub use schema::*;
