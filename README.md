**This project is still in experimental stage. DO NOT USE FOR PRODUCT.**

ruststep
=========

Crates for STEP (Standard for the Exchange of Product model data) written in pure Rust
aimed at replacing [stepcode](https://github.com/stepcode/stepcode).

| name | crates.io | docs.rs | master | description |
|:-----|:----------|:--------|:-------|-------------|
| espr |[![Crate](https://img.shields.io/crates/v/espr.svg)](https://crates.io/crates/espr) |[![docs.rs](https://docs.rs/espr/badge.svg)](https://docs.rs/espr) |[![cargo-doc](https://img.shields.io/badge/master-espr-blue)][espr-doc] |[EXPRESS Language (ISO 10303-11)][EXPRESS] Compiler|
| espr-derive |[![Crate](https://img.shields.io/crates/v/espr-derive.svg)](https://crates.io/crates/espr-derive) |[![docs.rs](https://docs.rs/espr-derive/badge.svg)](https://docs.rs/espr-derive) |[![cargo-doc](https://img.shields.io/badge/master-espr_derive-blue)][espr-derive-doc] |proc-macro for running espr compiler|
| ruststep | [![Crate](https://img.shields.io/crates/v/ruststep.svg)](https://crates.io/crates/ruststep) | [![docs.rs](https://docs.rs/ruststep/badge.svg)](https://docs.rs/ruststep) |[![cargo-doc](https://img.shields.io/badge/master-ruststep-blue)][ruststep-doc]|Serialize/Deserialize STEP files|
| ruststep-derive | [![Crate](https://img.shields.io/crates/v/ruststep-derive.svg)](https://crates.io/crates/ruststep-derive) | [![docs.rs](https://docs.rs/ruststep-derive/badge.svg)](https://docs.rs/ruststep-derive) |[![cargo-doc](https://img.shields.io/badge/master-ruststep--derive-blue)][ruststep-derive-doc]|proc-macro helper crate|

[espr-doc]: https://ricosjp.github.io/ruststep/espr/index.html
[espr-derive-doc]: https://ricosjp.github.io/ruststep/espr_derive/index.html
[ruststep-doc]: https://ricosjp.github.io/ruststep/ruststep/index.html
[ruststep-derive-doc]: https://ricosjp.github.io/ruststep/ruststep_derive/index.html
[EXPRESS]: https://www.iso.org/standard/38047.html

What is STEP?
--------------

- STEP is a set of data serialize formats, schema language, and common schemas.
- Data serialize format is called **exchange structure** in ISO document, but usually called **STEP file**,
  They are serialized as ASCII text (ISO-10303-21, usually with extension `*.step`, `*.stp` or `*.p21`) or XML (ISO-10303-28).
- Schema language is called **EXPRESS**. EXPRESS file is usually named with extension `*.exp`.
- Many common schemas are defined in ISO-10303 by EXPRESS language.
  - [schemas](./schemas) contains copies
  - Application Protocol (AP) is a class of defined schemas, and the main target of this project.
  - AP203 is most famous one in CAD (computer-aided design) applications.

### Rosetta Stone for web developers

|                 | Protocol Buffers                                           | STEP (ISO 10303)                                         |
|:----------------|:-----------------------------------------------------------|:---------------------------------------------------------|
| Schema Language | [Protocol Buffers Version 3 Language Specification][pbspec]| EXPRESS Language (ISO 10303-11)                          |
| Schema file     | `*.proto` file                                             | `*.exp` file                                             |
| Data            | [Encoded Binary data][pbencoding]                          | "Exchange structure", `*.step` file (ASCII, ISO 10303-21)|
| Compiler        | protoc                                                     | esprc                                                    |

[pbspec]: https://developers.google.com/protocol-buffers/docs/reference/proto3-spec
[pbencoding]: https://developers.google.com/protocol-buffers/docs/encoding

Why ruststep?
--------------

- STEP is not only for CAD
  - EXPRESS is a general data schema like [Protocol Buffers][pbspec]
  - Later ISOs also uses EXPRESS
    - [ISO 13584 "Industrial automation systems and integration - Parts library"](https://www.iso.org/standard/43423.html)
    - [ISO 13399 "Cutting tool data representation and exchange"](https://www.iso.org/standard/36757.html)
  - For computer-aided technologies (CAx) including:
    - computer-aided manufacturing (CAM)
    - computer-aided engineering (CAE)
    - product data management (PDM/EDM)
    - manufacturing resource planning (MRP)
    - enterprise resource planning (ERP)
- We have to generate many codes from EXPRESS schemas
  - Tables for SQL or NoSQL database, Object-Record Mapper (ORM)
  - on-wire, on-memory format
    - ASCII / XML are inefficient than binary format e.g. protocol buffers
- Extensible EXPRESS compiler is required
  - Like as protoc generates gRPC binding using gRPC-plugin

Roadmap
--------

### Released features

- 0.1.0
  - Minimal EXPRESS Compiler to generate Rust struct definitions
  - Deserialize STEP file (ASCII) to Rust struct

### TODO until 1.0 release

- Serialize Rust struct to STEP file (ASCII) https://github.com/ricosjp/ruststep/issues/13
- Translate rules in EXPRESS schema into Rust code https://github.com/ricosjp/ruststep/issues/43
- Stablize AST and IR representation of espr

### Planned features

- Binary format convertible into STEP ASCII and XML formats
- RDB support, ORM generation

License
--------
Copyright 2021 RICOS Co. Ltd.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

### Exception

The following directories are included for development purpose.
**They are not parts of this project**.

- [schemas](./schemas)
- [ruststep/tests/steps](./ruststep/tests/steps)

Contributor License Agreement (CLA)
----------------------------------

When you contribute (code, documentation, or anything else),
you have to be aware that your contribution is covered by the same Apache 2.0 License that is applied to ruststep itself.
This applies to all contributors, including those contributing on behalf of a company.
If you agree to its content, you simply have to click on the link posted by the [CLA assistant bot](https://github.com/CLAassistant) as a comment to the pull request.
Click it to check the CLA, then accept it on the following screen if you agree to it.
[CLA assistant](https://cla-assistant.io/) will save this decision for upcoming contributions and will notify you if there is any change to the CLA in the meantime.
