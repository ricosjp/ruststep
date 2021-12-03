use espr::{ast::SyntaxTree, codegen::rust::*, ir::IR};

const EXPRESS: &str = r#"
SCHEMA test_schema;
  ENTITY base SUPERTYPE OF (ONEOF (sub));
    x: REAL;
  END_ENTITY;

  ENTITY sub
     SUPERTYPE OF (subsub)
     SUBTYPE OF (base);
    y: REAL;
  END_ENTITY;

  ENTITY subsub SUBTYPE OF (sub);
    z: REAL;
  END_ENTITY;
END_SCHEMA;
"#;

#[test]
fn subsuper() {
    let st = SyntaxTree::parse(EXPRESS).unwrap();
    let ir = IR::from_syntax_tree(&st).unwrap();
    let tt = ir.to_token_stream(CratePrefix::External).to_string();

    let tt = rustfmt(tt);

    insta::assert_snapshot!(tt, @r###"
    pub mod test_schema {
        use ruststep::{
            as_holder, derive_more::*, error::Result, primitive::*, tables::*, Holder, TableInit,
        };
        use std::collections::HashMap;
        #[derive(Debug, Clone, PartialEq, Default, TableInit)]
        pub struct Tables {
            base: HashMap<u64, as_holder!(Base)>,
            sub: HashMap<u64, as_holder!(Sub)>,
            subsub: HashMap<u64, as_holder!(Subsub)>,
        }
        impl Tables {
            pub fn base_iter<'table>(&'table self) -> impl Iterator<Item = Result<Base>> + 'table {
                self.base
                    .values()
                    .cloned()
                    .map(move |value| value.into_owned(&self))
            }
            pub fn sub_iter<'table>(&'table self) -> impl Iterator<Item = Result<Sub>> + 'table {
                self.sub
                    .values()
                    .cloned()
                    .map(move |value| value.into_owned(&self))
            }
            pub fn subsub_iter<'table>(&'table self) -> impl Iterator<Item = Result<Subsub>> + 'table {
                self.subsub
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
            # [holder (field = base)]
            Base(Box<Base>),
            #[holder(use_place_holder)]
            # [holder (field = sub)]
            Sub(Box<Sub>),
        }
        #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
        # [holder (table = Tables)]
        # [holder (field = sub)]
        #[holder(generate_deserialize)]
        pub struct Sub {
            #[holder(use_place_holder)]
            pub base: Base,
            pub y: f64,
        }
        #[derive(Debug, Clone, PartialEq, Holder)]
        # [holder (table = Tables)]
        #[holder(generate_deserialize)]
        pub enum SubAny {
            #[holder(use_place_holder)]
            # [holder (field = sub)]
            Sub(Box<Sub>),
            #[holder(use_place_holder)]
            # [holder (field = subsub)]
            Subsub(Box<Subsub>),
        }
        impl Into<BaseAny> for SubAny {
            fn into(self) -> BaseAny {
                BaseAny::SubAny(Box::new(self))
            }
        }
        #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
        # [holder (table = Tables)]
        # [holder (field = subsub)]
        #[holder(generate_deserialize)]
        pub struct Subsub {
            #[holder(use_place_holder)]
            pub sub: Sub,
            pub z: f64,
        }
        impl Into<SubAny> for Subsub {
            fn into(self) -> SubAny {
                SubAny::Subsub(Box::new(self))
            }
        }
    }
    "###);
}
