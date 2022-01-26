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

  ENTITY c_entity;
    x: REAL;
  END_ENTITY;

  TYPE c = c_entity;
  END_TYPE;

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
            c_entity: HashMap<u64, as_holder!(CEntity)>,
            a: HashMap<u64, as_holder!(A)>,
            c: HashMap<u64, as_holder!(C)>,
        }
        impl Tables {
            pub fn c_entity_holders(&self) -> &HashMap<u64, as_holder!(CEntity)> {
                &self.c_entity
            }
            pub fn a_holders(&self) -> &HashMap<u64, as_holder!(A)> {
                &self.a
            }
            pub fn c_holders(&self) -> &HashMap<u64, as_holder!(C)> {
                &self.c
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
        #[derive(
            Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, :: ruststep_derive :: Holder,
        )]
        # [holder (table = Tables)]
        # [holder (field = c)]
        #[holder(generate_deserialize)]
        pub struct C(#[holder(use_place_holder)] pub CEntity);
        #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
        # [holder (table = Tables)]
        # [holder (field = c_entity)]
        #[holder(generate_deserialize)]
        pub struct CEntity {
            pub x: f64,
        }
    }
    "###);
}
