use espr::{ast::SyntaxTree, codegen::rust::*, ir::IR};

use anyhow::*;
use std::{
    io::Write,
    process::{Command, Stdio},
};

fn rustfmt(tt: String) -> Result<String> {
    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(tt.as_bytes())
            .expect("Failed to write to stdin");
    });
    let output = child.wait_with_output()?;
    Ok(String::from_utf8(output.stdout)?)
}

#[test]
fn simple() {
    let input = r#"
    SCHEMA test_schema;
      ENTITY a;
        x: REAL;
        y: REAL;
      END_ENTITY;

      ENTITY b;
        z: REAL;
        a: a;
      END_ENTITY;
    END_SCHEMA;
    "#
    .trim();

    let st = SyntaxTree::parse(input).unwrap();
    let ir = IR::from_syntax_tree(&st).unwrap();
    let tt = ir.to_token_stream(CratePrefix::External).to_string();

    let tt = rustfmt(tt).unwrap();

    insta::assert_snapshot!(tt, @r###"
    pub mod test_schema {
        use ruststep::{as_holder, error::Result, primitive::*, tables::*, Holder, TableInit};
        use std::collections::HashMap;
        #[derive(Debug, Clone, PartialEq, Default, TableInit)]
        pub struct Tables {
            a: HashMap<u64, as_holder!(A)>,
            b: HashMap<u64, as_holder!(B)>,
        }
        impl Tables {
            pub fn a_iter<'table>(&'table self) -> impl Iterator<Item = Result<A>> + 'table {
                self.a
                    .values()
                    .cloned()
                    .map(move |value| value.into_owned(&self))
            }
            pub fn b_iter<'table>(&'table self) -> impl Iterator<Item = Result<B>> + 'table {
                self.b
                    .values()
                    .cloned()
                    .map(move |value| value.into_owned(&self))
            }
        }
        #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
        # [holder (table = Tables)]
        # [holder (field = a)]
        #[holder(generate_deserialize)]
        pub struct A {
            pub x: f64,
            pub y: f64,
        }
        #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
        # [holder (table = Tables)]
        # [holder (field = b)]
        #[holder(generate_deserialize)]
        pub struct B {
            pub z: f64,
            #[holder(use_place_holder)]
            pub a: A,
        }
    }
    "###);
}