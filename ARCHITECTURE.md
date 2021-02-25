Architecture
=============

This document describes the high-level architecture of ruststep project for new developers.

EXPRESS Language compiler (esprc)
----------------------------------

<img src="./espr-overview.svg" width=500 />

- Compilation in `espr` crate has three phases
  - **Tokenize**: Read the EXPRESS language files (usually named as `*.exp`), and parse into abstract syntax tree (AST).
  - **Legalize**: Convert AST to IR (intermediate representation) to ready the following code generation
    - Look up the references in AST
    - Resolve sub/super relations between entities
  - **Code Generation**: Create Rust module which will be used for STEP file I/O

STEP file I/O
--------------
To be written...
