Architecture
=============

This document aims to show an overview of this repository for new developers before looking into the reference of each crates.

EXPRESS Language compiler (espr crate)
---------------------------------------

Compilation in `espr` crate has three phases:

- **Tokenize**: Read the EXPRESS language files (usually named as `*.exp`), and parse into abstract syntax tree (AST).
  - [espr::ast][espr_ast] defines AST types to be used in [espr::parser][espr_parser]
  - [espr::parser][espr_parser] parses EXPRESS input using [nom][nom] parser combinator.
- **Legalize**: Convert AST to IR (intermediate representation) to ready the following code generation
  - [espr::ir][espr_ir] defines IR types, and they implements `Legalize` trait for legalizing from AST
- **Code Generation**: Create a submodule of [ruststep][ruststep] based on IR
  - [espr::codegen][espr_codegen] defines functions for generating Rust code from IR.

[nom]: https://docs.rs/nom/latest/nom/

[espr]:         https://ricosjp.github.io/ruststep/espr/index.html
[espr_ast]:     https://ricosjp.github.io/ruststep/espr/ast/index.html
[espr_parser]:  https://ricosjp.github.io/ruststep/espr/parser/index.html
[espr_ir]:      https://ricosjp.github.io/ruststep/espr/ir/index.html
[espr_codegen]: https://ricosjp.github.io/ruststep/espr/codegen/index.html

Rust code generation phase consists of two steps:

- Generate *abstract* Rust code by `esprc` (executable in [espr][espr] crate)
- Generate *concrete* Rust code by [ruststep-derive][ruststep-derive] crate

We have to generate large lines of codes, but most of them are based on a few information.
The first step of codegen is responsible for passing necessary information from espr to proc-macro,
and the second is responsible for generate actual code.

[ruststep-derive]: https://ricosjp.github.io/ruststep/ruststep_derive/index.html

STEP file I/O
--------------

Exchange structure, or STEP file is a data serialization format consists of following sections:

- Header
- Anchor (optional)
- Reference (optional)
- Data
- Signature (optional)

Data section consists of data definitions like following (from ISO-10303-21):

```
#2 = WIDGET(99, 99999, 'ABC', 'ABCDEFG', .T., .F., 9., 1.2345, @10, @PI);
```

This defines an entity instance `#2`, whose type is `WIDGET` and its value consists of values in the parenthesis.
`@10` is a reference to the instance `#10`, and it may be another type.
`.XXX.` is an enum value, e.g. `.T.` means `True` boolean value.
The definition of `WIDGET` type is not contained in the STEP file, and will be given in external EXPRESS schemas.

Because of the non-exclusive reference between entity instances, the data structure expressed by the exchange structure must be a graph.
Since the STEP files are usually very large file, we hope to parse them in streaming way without loading entire data on memory.
To realize these requirements, STEP file is parsed into several tables. Roughly, following a data section

```
DATA;
  #1 = A(1, 2);
  #2 = A(3, 4);
  #3 = B(5, @1);
  #4 = B(6, @1);
  #5 = B(5, @2);
ENDSEC;
```

will be parsed into two tables:

| Table A | value1 | value2 |
|:--------|:-------|:-------|
| `#1`    | 1      | 2      |
| `#2`    | 3      | 4      |

| Table B | value3 | @A     |
|:--------|:-------|:-------|
| `#3`    | 5      | 1      |
| `#4`    | 6      | 1      |
| `#5`    | 5      | 2      |

We may be able to built dynamically these tables only from a STEP file,
however, we create these tables statically from EXPRESS schema.

```
ENTITY a;
  x: INTEGER;
  y: INTEGER;
END_ENTITY;

ENTITY b;
  z: INTEGER;
  w: a;
END_ENTITY;
```

Then the each columns of the tables become statically typed:

| Table (a) | x (int) | y (int) |
|:----------|:--------|:--------|
| `#1`      | 1       | 2       |
| `#2`      | 3       | 4       |

| Table (b) | z (int) | w (a) |
|:----------|:--------|:------|
| `#3`      | 5       | 1     |
| `#4`      | 6       | 1     |
| `#5`      | 5       | 2     |
