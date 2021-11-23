How to add a test
------------------

Tests in this directory use [insta](https://github.com/mitsuhiko/insta) snapshot testing library.
Please copy following template and fill EXPRESS schema and test name:

```
use espr::{ast::SyntaxTree, codegen::rust::*, ir::IR};

const EXPRESS: &str = r#"
{{ Add EXPRESS schema you want to test }}
"#;

#[test]
fn {{ test name }}() {
    let st = SyntaxTree::parse(EXPRESS).unwrap();
    let ir = IR::from_syntax_tree(&st).unwrap();
    let tt = ir.to_token_stream(CratePrefix::External).to_string();

    let tt = rustfmt(tt);

    insta::assert_snapshot!(tt, @"");
}
```

Here, you can run `cargo test`, but it will fail since the snapshot is empty.
To fill the snapshot, you can use `cargo-insta` CLI tool.

```
cargo install cargo-insta
```

And then,

```
cargo insta review
```

This command ask you to accept/reject the generated result.
See also https://insta.rs/docs/cli/ for detail usage.
