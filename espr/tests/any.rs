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
        use ruststep::{as_holder, derive_more::*, primitive::*, Holder, TableInit};
        use std::collections::HashMap;
        #[derive(Debug, Clone, PartialEq, Default, TableInit)]
        pub struct Tables {
            base: HashMap<u64, as_holder!(Base)>,
            sub1: HashMap<u64, as_holder!(Sub1)>,
            sub2: HashMap<u64, as_holder!(Sub2)>,
        }
        impl Tables {
            pub fn base_holders(&self) -> &HashMap<u64, as_holder!(Base)> {
                &self.base
            }
            pub fn sub1_holders(&self) -> &HashMap<u64, as_holder!(Sub1)> {
                &self.sub1
            }
            pub fn sub2_holders(&self) -> &HashMap<u64, as_holder!(Sub2)> {
                &self.sub2
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
            Base(Box<Base>),
            #[holder(use_place_holder)]
            Sub1(Box<Sub1>),
            #[holder(use_place_holder)]
            Sub2(Box<Sub2>),
        }
        impl Into<BaseAny> for Base {
            fn into(self) -> BaseAny {
                BaseAny::Base(Box::new(self))
            }
        }
        impl Into<BaseAny> for Sub1 {
            fn into(self) -> BaseAny {
                BaseAny::Sub1(Box::new(self.into()))
            }
        }
        impl Into<BaseAny> for Sub2 {
            fn into(self) -> BaseAny {
                BaseAny::Sub2(Box::new(self.into()))
            }
        }
        impl AsRef<Base> for BaseAny {
            fn as_ref(&self) -> &Base {
                match self {
                    BaseAny::Base(x) => x.as_ref(),
                    BaseAny::Sub1(x) => (**x).as_ref(),
                    BaseAny::Sub2(x) => (**x).as_ref(),
                }
            }
        }
        #[derive(
            Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
        )]
        # [holder (table = Tables)]
        # [holder (field = sub1)]
        #[holder(generate_deserialize)]
        pub struct Sub1 {
            #[as_ref]
            #[as_mut]
            #[deref]
            #[deref_mut]
            #[holder(use_place_holder)]
            pub base: Base,
            pub y1: f64,
        }
        #[derive(
            Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
        )]
        # [holder (table = Tables)]
        # [holder (field = sub2)]
        #[holder(generate_deserialize)]
        pub struct Sub2 {
            #[as_ref]
            #[as_mut]
            #[deref]
            #[deref_mut]
            #[holder(use_place_holder)]
            pub base: Base,
            pub y2: f64,
        }
    }
    "###);
}
