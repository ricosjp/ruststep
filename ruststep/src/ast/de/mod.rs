//! Implementation of [serde::de] traits for AST structs
//!
//! Mapping to serde data model
//! ----------------------------
//!
//! [Implementing a Deserializer](https://serde.rs/impl-deserializer.html) page of [serde manual](https://serde.rs/) says
//! > The deserializer is responsible for mapping the input data
//! > into [Serde's data model](https://serde.rs/data-model.html) by invoking exactly one of the methods
//! > on the Visitor that it receives.
//!
//! [serde::de::Deserializer] trait is implemented for [Parameter] and [Record].
//!
//! - [Record] is mapped to `map` in serde data model through [RecordDeserializer],
//! - [Parameter] is mapped as following table:
//!
//! | Parameter   | serde data model |
//! |:------------|:-----------------|
//! | Integer     | i64              |
//! | Real        | f64              |
//! | String      | string           |
//! | List        | seq              |
//! | NotProvided | option (always none)|
//! | Omitted     | option (always none)|
//! | Enumeration | unit_variant (through [serde::de::value::StringDeserializer])|
//! | Typed       | map (through [RecordDeserializer])|
//! | Ref         | newtype_variant  |
//!
//! Be sure that this mapping is not only for espr-generated structs.
//! This can be used with other Rust structs using `serde_derive::Deserialize` custom derive:
//!
//! ```text
//! ┌────────────────────┐
//! │ Exchange Structure │
//! └─┬──────────────────┘
//!   │ Deserialier trait  ◄── Implemented here
//! ┌─▼────────────────┐
//! │ serde data model │
//! └─┬────┬───────────┘
//!   │    │ ruststep_derive::Deserialize
//!   │ ┌──▼─────────────────────────┐
//!   │ │ espr-generated Rust struct │
//!   │ └────────────────────────────┘
//!   │ serde_derive::Deserialize
//! ┌─▼─────────────────┐
//! │ Other Rust struct │
//! └───────────────────┘
//! ```
//!
//! Following examples show how it works with `serde_derive::Deserialize`:
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
//! ## Parameter::Enumeration
//!
//! ```
//! use ruststep::ast::*;
//! use serde::Deserialize;
//! use std::str::FromStr;
//!
//! let p = Parameter::from_str(".A.").unwrap();
//!
//! #[derive(Debug, PartialEq, Deserialize)]
//! enum E {
//!   A,
//!   B
//! }
//! assert_eq!(E::deserialize(&p).unwrap(), E::A);
//! ```
//!
//! ## Parameter::Ref
//!
//! ```
//! use ruststep::ast::*;
//! use serde::Deserialize;
//! use std::str::FromStr;
//!
//! let p = Parameter::from_str("#12").unwrap();
//!
//! #[derive(Debug, PartialEq, Deserialize)]
//! enum Id {
//!   #[serde(rename = "Entity")] // "Entity" is keyword for entity reference
//!   E(usize),
//!   #[serde(rename = "Value")]  // "Value" is keyword for value reference
//!   V(usize),
//! }
//! assert_eq!(Id::deserialize(&p).unwrap(), Id::E(12));
//! ```
//!

mod deserializer;
mod parameter;

pub use deserializer::*;
pub use parameter::*;

#[cfg(doc)]
use crate::ast::*;
