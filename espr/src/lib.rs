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

#![deny(rustdoc::broken_intra_doc_links)]

pub mod ast;
pub mod codegen;
pub mod ir;
pub mod parser;
