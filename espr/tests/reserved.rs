use espr::{ast::SyntaxTree, codegen::rust::*, ir::IR};

const EXPRESS: &str = r#"
SCHEMA test_schema;
  ENTITY loop;
    a: REAL;
  END_ENTITY;

  ENTITY a;
    z: REAL;
    a_loop: loop;
  END_ENTITY;

  TYPE b = loop;
  END_TYPE;

  ENTITY c;
    loop: b;
  END_ENTITY;
END_SCHEMA;
"#;

#[test]
fn reserved_keyword() {
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
            r#loop: HashMap<u64, as_holder!(Loop)>,
            a: HashMap<u64, as_holder!(A)>,
            c: HashMap<u64, as_holder!(C)>,
            b: HashMap<u64, as_holder!(B)>,
        }
        impl Tables {
            pub fn loop_holders(&self) -> &HashMap<u64, as_holder!(Loop)> {
                &self.r#loop
            }
            pub fn a_holders(&self) -> &HashMap<u64, as_holder!(A)> {
                &self.a
            }
            pub fn c_holders(&self) -> &HashMap<u64, as_holder!(C)> {
                &self.c
            }
            pub fn b_holders(&self) -> &HashMap<u64, as_holder!(B)> {
                &self.b
            }
        }
        #[derive(
            Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
        )]
        # [holder (table = Tables)]
        # [holder (field = b)]
        #[holder(generate_deserialize)]
        pub struct B(#[holder(use_place_holder)] pub Loop);
        #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
        # [holder (table = Tables)]
        # [holder (field = r#loop)]
        #[holder(generate_deserialize)]
        pub struct Loop {
            pub a: f64,
        }
        #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
        # [holder (table = Tables)]
        # [holder (field = a)]
        #[holder(generate_deserialize)]
        pub struct A {
            pub z: f64,
            #[holder(use_place_holder)]
            pub a_loop: Loop,
        }
        #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
        # [holder (table = Tables)]
        # [holder (field = c)]
        #[holder(generate_deserialize)]
        pub struct C {
            #[holder(use_place_holder)]
            pub r#loop: B,
        }
    }
    "###);
}
