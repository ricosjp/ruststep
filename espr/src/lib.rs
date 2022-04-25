//! A crate for implementing EXPRESS language compiler and related tools
//!
//! Overview
//! --------
//!
//! ```text
//! ┌────────────────┐
//! │ EXPRESS Schema │
//! └──┬─────────────┘
//!    │ Tokenize
//! ┌──▼─────────────────────────┐
//! │ Abstract Syntax Tree (AST) │
//! └──┬─────────────────────────┘
//!    │ Legalize
//! ┌──▼───────────────────────────────┐
//! │ Intermediate Representation (IR) │
//! └──┬───────────────────────────────┘ │
//!    │ Code Generation                 │ espr's responsibility
//! ┌──▼────────────────────────┐        ▼
//! │ Rust code with proc-macro │  ─────────────────────────────────
//! └──┬────────────────────────┘
//!    │ proc-macro
//! ┌──▼──────────────┐ call ┌──────────┐
//! │ Final Rust code ├──────► ruststep │
//! └─────────────────┘      └──────────┘
//! ```
//!
//! - Tokenize
//!   - Read EXPRESS input and parse into abstract syntax tree (AST)
//!   - [ast] module defines AST structs
//!   - [parser] module defines parser combinator for parsing EXPRESS language
//! - Legalize
//!   - Convert AST to IR (intermediate representation) to ready the following code generation
//!   - [ir] module defines IR structs, and they implements [ir::Legalize] trait for legalizing from AST
//! - Code Generation
//!   - [codegen::rust] module generates Rust code from IR
//!
//! Introduction to STEP
//! ---------------------
//!
//! As we describe in [README](https://github.com/ricosjp/ruststep#readme),
//! STEP consists of three components:
//!
//! - Schema language called EXPRESS
//! - Data format called "exchange structure" or "STEP file"
//! - Schemas, e.g. AP203 for CAD components
//!
//! This short section shows basic concept of EXPRESS schemas and exchange structures
//! for developers new to STEP ecosystem.
//!
//! EXPRESS language, standardized as [ISO-10303-11](https://www.iso.org/standard/38047.html),
//! is a schema language as a part of STEP ecosystem.
//!
//! ```text
//! SCHEMA my_first_schema;
//!   ENTITY a;
//!     x: REAL;
//!     y: REAL;
//!   END_ENTITY;
//!
//!   ENTITY b;
//!     z: REAL;
//!     a: a;
//!   END_ENTITY;
//! END_SCHEMA;
//! ```
//!
//! This example schema written in EXPRESS language defines
//! `my_first_schema` schema and two entities `a` and `b`.
//! Entity is a collection of primitive types or other entities similar to `struct` in Rust.
//!
//! Actual data will be stored in another data format called "exchange structure" or "STEP file",
//! standardized as [ISO-10303-21](https://www.iso.org/standard/63141.html).
//! Exchange structure consists of following sections:
//!
//! - HEADER
//! - ANCHOR (optional)
//! - REFERENCE (optional)
//! - DATA
//! - SIGNATURE (optional)
//!
//! A data section contains indexed instances:
//!
//! ```text
//! DATA;
//!   #1 = A(1.0, 2.0);
//!   #2 = B(3.0, A((4.0, 5.0)));
//!   #3 = B(6.0, #1);
//! ENDSEC;
//! ```
//!
//! This describes three instances, `#1` of type `A`, and `#2` and `#3` of type `B`.
//! Data section itself is independent from any schema.
//! The grammar of exchange structure allows any number of fields, e.g. `A(1.0, 2.0, 3.0)`.
//! What data must be contained in the data section is specified in its header section:
//!
//! ```text
//! HEADER;
//!   FILE_SCHEMA(('my_first_schema'));
//!   /* others */
//! ENDSEC;
//! ```
//!
//! The type `A` (`B`) corresponds to `a` (`b`) entity in schema.
//! Programs reading this exchange structure should reject invalid data
//! e.g. `A(1.0, 2.0, 3.0)` based on the schema specification,
//! and this crate aims to help creating such programs easily.
//!
//! Terms and definitions
//! -----------------------
//!
//! These are excerpts from ISO 10303-11 3.3 "Other terms and definitions"
//! to help understanding terms appears in this document.
//!
//! ### Basic terms
//!
//! | Section | Term      | Meaning |
//! |:--------|:----------|:--------|
//! | 3.3.22  | value     | a unit of data.|
//! | 3.3.4   | constant  | a named unit of data from a specified domain. The value cannot be modified.|
//! | 3.3.10  | instance  | a named value.|
//! | 3.3.5   | data type | a domain of values.|
//!
//! ### Terms about entity
//!
//! | Section | Term                              | Meaning |
//! |:--------|:----------------------------------|:--------|
//! | 3.3.6   | entity                            | a class of information defined by common properties.|
//! | 3.3.7   | entity data type                  | a representation of an entity. An entity data type establishes a domain of values defined by common attributes and constraints.|
//! | 3.3.8   | entity (data type) instance       | a named entity data type value. The name of an entity instance is used for referencing the instance.|
//! | 3.3.9   | (single) entity (data type) value | a unit of data which represents a unit of information within the class defined by an entity data type. It is a member of the domain established by this entity data type.|
//! | 3.3.16  | population                        | a collection of entity data type instances.|
//!
//! ### Advanced terms about entity
//!
//! | Section | Term                               | Meaning |
//! |:--------|:-----------------------------------|:--------|
//! | 3.3.1   | complex entity                     | a representation of an entity. A complex entity data type establishes a domain of values defined by the common attributes and constraints of an allowed combination of entity data types within a particular subtype/supertype graph.|
//! | 3.3.19  | simple entity instance             | a named unit of data which represents a unit of information within the class defined by an entity. It is a member of the domain established by a single entity data type.|
//! | 3.3.2   | complex entity instance            | a named complex entity data type value. The name of a complex entity instance is used for referencing the instance.|
//! | 3.3.3   | complex entity value               | a unit of data that represents a unit of information within the class defined by a complex entity data type. It is a member of the domain established by this complex entity data type.|
//! | 3.3.14  | partial complex entity             | a potential representation of an entity. A partial complex entity data type is a grouping of entity data types within a subtype/supertype graph which may form part or all of a complex entity data type.|
//! | 3.3.15  | partial complex entity value       | a value of a partial complex entity data type. This has no meaning on its own and must be combined with other partial complex entity values and a name to form a complex entity instance.|
//! | 3.3.11  | multi-leaf complex entity          | a complex entity data type that consists of more than one entity data types that do not have further subtypes within this complex entity data type.|
//! | 3.3.12  | multi-leaf complex entity instance | a named multi-leaf complex entity data type value. The name of a multi-leaf complex entity instance is used for referencing the instance.|
//! | 3.3.13  | multi-leaf complex entity value    | a unit of data that represents a unit of information within the class defined by a multi-leaf complex entity data type. It is a member of the domain established by this multi-leaf complex entity data type.|
//!
//! "data type" is ommitted here since "entity" and "entity data type"
//! indicate same one in most situation.
//!

#![deny(rustdoc::broken_intra_doc_links)]

pub mod ast;
pub mod codegen;
pub mod ir;
pub mod parser;
