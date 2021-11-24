use espr::{ast::SyntaxTree, codegen::rust::*, ir::IR};

const EXPRESS: &str = r#"
SCHEMA test_schema;
  ENTITY base SUPERTYPE OF (ONEOF (sub1, sub2));
    x: REAL;
  END_ENTITY;

  ENTITY sub1 SUBTYPE OF (base);
    y1: REAL;
  END_ENTITY;

  ENTITY sub2 SUBTYPE OF (base);
    y2: REAL;
  END_ENTITY;
END_SCHEMA;
"#;

#[test]
fn any() {
    let st = SyntaxTree::parse(EXPRESS).unwrap();
    let ir = IR::from_syntax_tree(&st).unwrap();
    let tt = ir.to_token_stream(CratePrefix::External).to_string();

    let tt = rustfmt(tt);

    insta::assert_snapshot!(tt, @r###"
    pub mod test_schema {
        use ruststep::{as_holder, error::Result, primitive::*, tables::*, Holder, TableInit};
        use std::collections::HashMap;
        #[derive(Debug, Clone, PartialEq, Default, TableInit)]
        pub struct Tables {
            base: HashMap<u64, as_holder!(Base)>,
            sub1: HashMap<u64, as_holder!(Sub1)>,
            sub2: HashMap<u64, as_holder!(Sub2)>,
        }
        impl Tables {
            pub fn base_iter<'table>(&'table self) -> impl Iterator<Item = Result<Base>> + 'table {
                self.base
                    .values()
                    .cloned()
                    .map(move |value| value.into_owned(&self))
            }
            pub fn sub1_iter<'table>(&'table self) -> impl Iterator<Item = Result<Sub1>> + 'table {
                self.sub1
                    .values()
                    .cloned()
                    .map(move |value| value.into_owned(&self))
            }
            pub fn sub2_iter<'table>(&'table self) -> impl Iterator<Item = Result<Sub2>> + 'table {
                self.sub2
                    .values()
                    .cloned()
                    .map(move |value| value.into_owned(&self))
            }
        }
        #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
        # [holder (table = Tables)]
        # [holder (field = base)]
        #[holder(generate_deserialize)]
        pub struct Base {
            pub x: f64,
        }
        #[derive(Debug, Clone, PartialEq, Holder)]
        # [holder (table = Tables)]
        #[holder(generate_deserialize)]
        pub enum BaseAny {
            #[holder(use_place_holder)]
            # [holder (field = sub1)]
            Sub1(Box<Sub1>),
            #[holder(use_place_holder)]
            # [holder (field = sub2)]
            Sub2(Box<Sub2>),
        }
        #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
        # [holder (table = Tables)]
        # [holder (field = sub1)]
        #[holder(generate_deserialize)]
        pub struct Sub1 {
            pub y1: f64,
        }
        #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
        # [holder (table = Tables)]
        # [holder (field = sub2)]
        #[holder(generate_deserialize)]
        pub struct Sub2 {
            pub y2: f64,
        }
    }
    "###);
}
