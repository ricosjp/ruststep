use espr::{ast::SyntaxTree, codegen::rust::*, ir::IR};

const EXPRESS: &str = r#"
SCHEMA test_schema;
  TYPE a = STRING;
  END_TYPE;

  TYPE b = ENUMERATION OF (
      are,
      sore,
      dore
    );
  END_TYPE;

  ENTITY c;
    a: a;
    b: b;
  END_ENTITY;

END_SCHEMA;
"#;

#[test]
fn type_decl() {
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
            c: HashMap<u64, as_holder!(C)>,
            a: HashMap<u64, as_holder!(A)>,
        }
        impl Tables {
            pub fn c_holders(&self) -> &HashMap<u64, as_holder!(C)> {
                &self.c
            }
            pub fn a_holders(&self) -> &HashMap<u64, as_holder!(A)> {
                &self.a
            }
        }
        #[derive(
            Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
        )]
        # [holder (table = Tables)]
        # [holder (field = a)]
        #[holder(generate_deserialize)]
        pub struct A(pub String);
        #[derive(Debug, Clone, PartialEq, :: serde :: Deserialize)]
        pub enum B {
            Are,
            Sore,
            Dore,
        }
        #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
        # [holder (table = Tables)]
        # [holder (field = c)]
        #[holder(generate_deserialize)]
        pub struct C {
            #[holder(use_place_holder)]
            pub a: A,
            pub b: B,
        }
    }
    "###);
}
