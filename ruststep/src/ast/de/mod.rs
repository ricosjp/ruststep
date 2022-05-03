//! Implementation of [serde::de] traits for AST structs

mod parameter;
mod record;
mod value;

pub use parameter::*;
pub use record::*;
pub use value::*;
