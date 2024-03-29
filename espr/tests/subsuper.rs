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
        use ruststep::{as_holder, derive_more::*, primitive::*, Holder, TableInit};
        use std::collections::HashMap;
        #[derive(Debug, Clone, PartialEq, Default, TableInit)]
        pub struct Tables {
            base: HashMap<u64, as_holder!(Base)>,
            sub: HashMap<u64, as_holder!(Sub)>,
            subsub: HashMap<u64, as_holder!(Subsub)>,
        }
        impl Tables {
            pub fn base_holders(&self) -> &HashMap<u64, as_holder!(Base)> {
                &self.base
            }
            pub fn sub_holders(&self) -> &HashMap<u64, as_holder!(Sub)> {
                &self.sub
            }
            pub fn subsub_holders(&self) -> &HashMap<u64, as_holder!(Subsub)> {
                &self.subsub
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
            Sub(Box<SubAny>),
        }
        impl Into<BaseAny> for Base {
            fn into(self) -> BaseAny {
                BaseAny::Base(Box::new(self))
            }
        }
        impl Into<BaseAny> for Sub {
            fn into(self) -> BaseAny {
                BaseAny::Sub(Box::new(self.into()))
            }
        }
        impl AsRef<Base> for BaseAny {
            fn as_ref(&self) -> &Base {
                match self {
                    BaseAny::Base(x) => x.as_ref(),
                    BaseAny::Sub(x) => (**x).as_ref(),
                }
            }
        }
        #[derive(
            Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
        )]
        # [holder (table = Tables)]
        # [holder (field = sub)]
        #[holder(generate_deserialize)]
        pub struct Sub {
            #[as_ref]
            #[as_mut]
            #[deref]
            #[deref_mut]
            #[holder(use_place_holder)]
            pub base: Base,
            pub y: f64,
        }
        #[derive(Debug, Clone, PartialEq, Holder)]
        # [holder (table = Tables)]
        #[holder(generate_deserialize)]
        pub enum SubAny {
            #[holder(use_place_holder)]
            Sub(Box<Sub>),
            #[holder(use_place_holder)]
            Subsub(Box<Subsub>),
        }
        impl Into<SubAny> for Sub {
            fn into(self) -> SubAny {
                SubAny::Sub(Box::new(self))
            }
        }
        impl Into<SubAny> for Subsub {
            fn into(self) -> SubAny {
                SubAny::Subsub(Box::new(self.into()))
            }
        }
        impl AsRef<Sub> for SubAny {
            fn as_ref(&self) -> &Sub {
                match self {
                    SubAny::Sub(x) => x.as_ref(),
                    SubAny::Subsub(x) => (**x).as_ref(),
                }
            }
        }
        impl AsRef<Base> for SubAny {
            fn as_ref(&self) -> &Base {
                match self {
                    SubAny::Sub(x) => AsRef::<Sub>::as_ref(x).as_ref(),
                    SubAny::Subsub(x) => AsRef::<Sub>::as_ref(x.as_ref()).as_ref(),
                }
            }
        }
        #[derive(
            Debug, Clone, PartialEq, :: derive_new :: new, Holder, AsRef, AsMut, Deref, DerefMut,
        )]
        # [holder (table = Tables)]
        # [holder (field = subsub)]
        #[holder(generate_deserialize)]
        pub struct Subsub {
            #[as_ref]
            #[as_mut]
            #[deref]
            #[deref_mut]
            #[holder(use_place_holder)]
            pub sub: Sub,
            pub z: f64,
        }
    }
    "###);
}
