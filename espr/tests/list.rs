use espr::{ast::SyntaxTree, codegen::rust::*, ir::IR};

const EXPRESS: &str = r#"
SCHEMA test_schema;
  ENTITY a;
    x: LIST [0:?] OF REAL;
  END_ENTITY;

  ENTITY b;
    a: LIST [0:?] OF a;
  END_ENTITY;

  TYPE c = LIST [0:?] OF REAL;
  END_TYPE;
  
  TYPE d = LIST [0:?] OF a;
  END_TYPE;
END_SCHEMA;
"#;

#[test]
fn list() {
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
            c: HashMap<u64, as_holder!(C)>,
            d: HashMap<u64, as_holder!(D)>,
        }
        impl Tables {
            pub fn a_holders(&self) -> &HashMap<u64, as_holder!(A)> {
                &self.a
            }
            pub fn b_holders(&self) -> &HashMap<u64, as_holder!(B)> {
                &self.b
            }
            pub fn c_holders(&self) -> &HashMap<u64, as_holder!(C)> {
                &self.c
            }
            pub fn d_holders(&self) -> &HashMap<u64, as_holder!(D)> {
                &self.d
            }
        }
        #[derive(Clone, Debug, PartialEq, AsRef, Deref, DerefMut, :: ruststep_derive :: Holder)]
        # [holder (table = Tables)]
        # [holder (field = c)]
        #[holder(generate_deserialize)]
        pub struct C(#[holder(use_place_holder)] pub Vec<f64>);
        #[derive(Clone, Debug, PartialEq, AsRef, Deref, DerefMut, :: ruststep_derive :: Holder)]
        # [holder (table = Tables)]
        # [holder (field = d)]
        #[holder(generate_deserialize)]
        pub struct D(#[holder(use_place_holder)] pub Vec<A>);
        #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
        # [holder (table = Tables)]
        # [holder (field = a)]
        #[holder(generate_deserialize)]
        pub struct A {
            pub x: Vec<f64>,
        }
        #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
        # [holder (table = Tables)]
        # [holder (field = b)]
        #[holder(generate_deserialize)]
        pub struct B {
            #[holder(use_place_holder)]
            pub a: Vec<A>,
        }
    }
    "###);
}
