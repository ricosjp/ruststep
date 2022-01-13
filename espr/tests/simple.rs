use espr::{ast::SyntaxTree, codegen::rust::*, ir::IR};

const EXPRESS: &str = r#"
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
"#;

#[test]
fn simple() {
    let st = SyntaxTree::parse(EXPRESS).unwrap();
    let ir = IR::from_syntax_tree(&st).unwrap();
    let tt = ir.to_token_stream(CratePrefix::External).to_string();

    let tt = rustfmt(tt);

    insta::assert_snapshot!(tt, @r###"
    pub mod test_schema {
        use ruststep::{as_holder, derive_more::*, primitive::*, Holder, TableInit};
        use std::collections::HashMap;
        #[derive(Debug, Clone, PartialEq, Default, TableInit)]
        pub struct Tables {
            a: HashMap<u64, as_holder!(A)>,
            b: HashMap<u64, as_holder!(B)>,
        }
        impl Tables {
            pub fn a_holders(&self) -> &HashMap<u64, as_holder!(A)> {
                &self.a
            }
            pub fn b_holders(&self) -> &HashMap<u64, as_holder!(B)> {
                &self.b
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
