//! Implementation of [serde::de] traits for AST structs
//!
//! Test mapping AST to serde data model,
//! without deserializing to espr-generated struct
//!
//! ## Parameter::Integer
//!
//! ```
//! use ruststep::ast::*;
//! use serde::Deserialize;
//!
//! let p = Parameter::Integer(2);
//! let a = i64::deserialize(&p).unwrap();
//! assert_eq!(a, 2);
//!
//! // can be deserialized as unsigned
//! let a = u64::deserialize(&p).unwrap();
//! assert_eq!(a, 2);
//!
//! // cannot be deserialized negative integer into unsigned
//! let p = Parameter::Integer(-2);
//! let a = i64::deserialize(&p).unwrap();
//! assert_eq!(a, -2);
//! assert!(u64::deserialize(&p).is_err());
//! ```
//!
//! ## Parameter::List
//!
//! ```
//! use std::str::FromStr;
//! use ruststep::ast::*;
//! use serde::Deserialize;
//!
//! let p = Parameter::from_str("(1, 2, 3)").unwrap();
//!
//! // As Vec<i32>
//! let a = Vec::<i32>::deserialize(&p).unwrap();
//! assert_eq!(a, vec![1, 2, 3]);
//!
//! // As user-defined struct
//! #[derive(Debug, Clone, PartialEq, Deserialize)]
//! struct A {
//!     x: i32,
//!     y: i32,
//!     z: i32,
//! }
//! let a = A::deserialize(&p).unwrap();
//! assert_eq!(a, A { x: 1, y: 2, z: 3 });
//! ```
//!
//! ## Parameter::Typed
//!
//! ```
//! use std::{str::FromStr, collections::HashMap};
//! use ruststep::ast::*;
//! use serde::Deserialize;
//!
//! // Regarded as a map `{ "A": [1, 2] }` in serde data model
//! let p = Parameter::from_str("A((1, 2))").unwrap();
//!
//! // Map can be deserialize as a hashmap
//! assert_eq!(
//!     HashMap::<String, Vec<i32>>::deserialize(&p).unwrap(),
//!     maplit::hashmap! {
//!         "A".to_string() => vec![1, 2]
//!     }
//! );
//!
//! // Map in serde cannot be deserialize to following struct
//! // using `serde_derive::Deserialize`.
//! #[derive(Debug, Clone, PartialEq, Deserialize)]
//! struct A {
//!     x: i32,
//!     y: i32,
//! }
//! assert!(A::deserialize(&p).is_err());
//!
//! // Map in serde can be interpreted as Rust field
//! #[derive(Debug, Clone, PartialEq, Deserialize)]
//! struct X {
//!     #[serde(rename = "A")]
//!     a: Vec<i32>,
//! }
//! assert_eq!(X::deserialize(&p).unwrap(), X { a: vec![1, 2] });
//! ```
//!
//! ## Parameter::NotProvided
//!
//! ```
//! use ruststep::ast::*;
//! use serde::Deserialize;
//!
//! let p = Parameter::NotProvided;
//! assert_eq!(Option::<i64>::deserialize(&p).unwrap(), None);
//! ```
//!
//! ## Parameter::Omitted
//!
//! ```
//! use ruststep::ast::*;
//! use serde::Deserialize;
//!
//! let p = Parameter::Omitted;
//! assert_eq!(Option::<i64>::deserialize(&p).unwrap(), None);
//! ```
//!

mod parameter;
mod record;
mod value;

pub use parameter::*;
pub use record::*;
pub use value::*;
