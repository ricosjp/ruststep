use espr::{ast::SyntaxTree, codegen::rust::*, ir::IR};

const EXPRESS: &str = r#"
SCHEMA test_schema;

ENTITY pet
  ABSTRACT SUPERTYPE OF (ONEOF(cat, rabbit, dog) );
  name : STRING;
END_ENTITY;

ENTITY cat
  SUBTYPE OF (pet);
  mike: BOOLEAN;
  male: BOOLEAN;
END_ENTITY;

ENTITY rabbit
  SUBTYPE OF (pet);
END_ENTITY;

ENTITY dog
  SUBTYPE OF (pet);
END_ENTITY;

END_SCHEMA;
"#;

#[test]
fn sub_super_type() {
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
            pet: HashMap<u64, as_holder!(Pet)>,
            cat: HashMap<u64, as_holder!(Cat)>,
            rabbit: HashMap<u64, as_holder!(Rabbit)>,
            dog: HashMap<u64, as_holder!(Dog)>,
        }
        impl Tables {
            pub fn pet_iter<'table>(&'table self) -> impl Iterator<Item = Result<Pet>> + 'table {
                self.pet
                    .values()
                    .cloned()
                    .map(move |value| value.into_owned(&self))
            }
            pub fn cat_iter<'table>(&'table self) -> impl Iterator<Item = Result<Cat>> + 'table {
                self.cat
                    .values()
                    .cloned()
                    .map(move |value| value.into_owned(&self))
            }
            pub fn rabbit_iter<'table>(&'table self) -> impl Iterator<Item = Result<Rabbit>> + 'table {
                self.rabbit
                    .values()
                    .cloned()
                    .map(move |value| value.into_owned(&self))
            }
            pub fn dog_iter<'table>(&'table self) -> impl Iterator<Item = Result<Dog>> + 'table {
                self.dog
                    .values()
                    .cloned()
                    .map(move |value| value.into_owned(&self))
            }
        }
        #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
        # [holder (table = Tables)]
        # [holder (field = pet)]
        #[holder(generate_deserialize)]
        pub struct Pet {
            pub name: String,
        }
        #[derive(Debug, Clone, PartialEq, Holder)]
        # [holder (table = Tables)]
        #[holder(generate_deserialize)]
        pub enum PetAny {
            #[holder(use_place_holder)]
            # [holder (field = pet)]
            Pet(Box<Pet>),
            #[holder(use_place_holder)]
            # [holder (field = cat)]
            Cat(Box<Cat>),
            #[holder(use_place_holder)]
            # [holder (field = rabbit)]
            Rabbit(Box<Rabbit>),
            #[holder(use_place_holder)]
            # [holder (field = dog)]
            Dog(Box<Dog>),
        }
        #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
        # [holder (table = Tables)]
        # [holder (field = cat)]
        #[holder(generate_deserialize)]
        pub struct Cat {
            #[holder(use_place_holder)]
            pub pet: Pet,
            pub mike: bool,
            pub male: bool,
        }
        impl Into<PetAny> for Cat {
            fn into(self) -> PetAny {
                PetAny::Cat(Box::new(self))
            }
        }
        #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
        # [holder (table = Tables)]
        # [holder (field = rabbit)]
        #[holder(generate_deserialize)]
        pub struct Rabbit {
            #[holder(use_place_holder)]
            pub pet: Pet,
        }
        impl Into<PetAny> for Rabbit {
            fn into(self) -> PetAny {
                PetAny::Rabbit(Box::new(self))
            }
        }
        #[derive(Debug, Clone, PartialEq, :: derive_new :: new, Holder)]
        # [holder (table = Tables)]
        # [holder (field = dog)]
        #[holder(generate_deserialize)]
        pub struct Dog {
            #[holder(use_place_holder)]
            pub pet: Pet,
        }
        impl Into<PetAny> for Dog {
            fn into(self) -> PetAny {
                PetAny::Dog(Box::new(self))
            }
        }
    }
    "###);
}
