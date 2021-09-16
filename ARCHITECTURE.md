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

STEP file I/O (ruststep crate)
-------------------------------

To understand the architecture of ruststep, we start from the STEP file structure.

### STEP file format

Exchange structure, as known as "STEP file" defined in [ISO-10303-21][ISO-10303-21]
is a data serialization format consists of following sections:
[ISO-10303-21]: https://www.iso.org/standard/63141.html

- Header
- Anchor (optional)
- Reference (optional)
- Data
- Signature (optional)

Data section consists of data definitions like following (from [ISO-10303-21][ISO-10303-21]):

```
#2 = WIDGET(99, 99999, 'ABC', 'ABCDEFG', .T., .F., 9., 1.2345, @10, @PI);
```

This defines an entity instance `#2`, whose type is `WIDGET` and its value consists of values in the parenthesis.
`@10` is a reference to the instance `#10`, and it may be another type.
`.XXX.` is an enum value, e.g. `.T.` means `True` boolean value.
The definition of `WIDGET` type is not contained in the STEP file, and will be given in external EXPRESS schemas.

### Deserialize STEP file

Let us consider a simple EXPRESS schema:

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

Corresponding data section in STEP file will be something like following:

```
DATA;
  #1 = A(1, 2);
  #2 = A(3, 4);
  #3 = B(5, @1);
  #4 = B(6, @1);
  #5 = B(7, @2);
  #6 = B(8, A((9, 10)));
ENDSEC;
```

In this example, `#3` and `#4` has reference to `#1`.
There will exist non-exclusive reference between entity instances generally, and thus the data must be regarded as a graph.

ruststep will parse this data section into following tables:

| Table (a) | x (i64) | y (i64) |
|:----------|:--------|:--------|
| `#1`      | 1       | 2       |
| `#2`      | 3       | 4       |

| Table (b) | z (i64) | w (a)    |
|:----------|:--------|:---------|
| `#3`      | 5       | @1       |
| `#4`      | 6       | @1       |
| `#5`      | 7       | @2       |
| `#6`      | 8       | A(9, 10) |

Each columns are defined by EXPRESS schema.
`x`, `y`, and `z` are specified as integer in EXPRESS, and will be treated as `i64` in Rust code.
The simple types in EXPRESS are mapped into Rust primitive types.
The ENTITY `a` will be treated as a Rust struct like

```rust
struct A {
  x: i64,
  y: i64,
}
```

The ENTITY `b` has to support both reference and inline struct like as `#4` and `#6`.
For this purpose, [PlaceHolder][PlaceHolder] exists:
[PlaceHolder]: https://ricosjp.github.io/ruststep/ruststep/place_holder/enum.PlaceHolder.html

```rust
pub enum PlaceHolder<T> {
    // for `@1`
    Ref(RValue),
    // for `A(9, 10)`
    Owned(T),
}
```

Then following two Rust structs will be defined:

```rust
struct B {
    z: i64,
    w: A,
}
struct BHolder {
    z: i64,
    w: PlaceHolder<AHolder>,
}
```

There also a function `into_owned(BHolder) -> B` in [Holder][Holder] trait.
`AHolder` will also be introduced to keep consistency.
These are automated by `#[derive(ruststep_derive::Holder)]` proc-macro.
[Holder]: https://ricosjp.github.io/ruststep/ruststep/tables/trait.Holder.html
