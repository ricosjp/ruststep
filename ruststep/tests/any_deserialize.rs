use nom::Finish;
use ruststep::{ast::*, error::*, parser::exchange, place_holder::*, tables::*};
use ruststep_derive::as_holder;
use serde::{de, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Table {
    base: HashMap<u64, as_holder!(Base)>,
    sub1: HashMap<u64, as_holder!(Sub1)>,
    sub2: HashMap<u64, as_holder!(Sub2)>,
}

impl Table {
    // ```
    // #1 = BASE(1.0);
    // #2 = SUB_1(BASE((1.0)), 2.0);
    // #3 = SUB_2(#1, 2.0);
    // ```
    fn example() -> Self {
        let mut table = Self::default();
        table.base.insert(1, BaseHolder { x: 1.0 });
        table.sub1.insert(
            2,
            Sub1Holder {
                base: BaseHolder { x: 1.0 }.into(),
                y1: 2.0,
            },
        );
        table.sub2.insert(
            2,
            Sub2Holder {
                base: RValue::Entity(1).into(),
                y2: 4.0,
            },
        );
        table
    }
}

#[derive(Clone, Debug, PartialEq, ruststep_derive::Holder)]
#[holder(table = Table)]
#[holder(field = base)]
pub struct Base {
    x: f64,
}

impl<'de> de::Deserialize<'de> for BaseHolder {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_tuple_struct("BASE", 1, BaseHolderVisitor {})
    }
}

pub struct BaseHolderVisitor;

impl<'de> ::serde::de::Visitor<'de> for BaseHolderVisitor {
    type Value = BaseHolder;
    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(formatter, "BASE")
    }

    fn visit_seq<A>(self, mut seq: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: ::serde::de::SeqAccess<'de>,
    {
        if let Some(size) = seq.size_hint() {
            if size != 1 {
                use ::serde::de::Error;
                return Err(A::Error::invalid_length(size, &self));
            }
        }
        let x = seq.next_element()?.unwrap();
        Ok(BaseHolder { x })
    }

    // Entry point for Record or Parameter::Typed
    fn visit_map<A>(self, mut map: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: ::serde::de::MapAccess<'de>,
    {
        let key: String = map
            .next_key()?
            .expect("Empty map cannot be accepted as ruststep Holder"); // this must be a bug, not runtime error
        if key != "BASE" {
            use ::serde::de::{Error, Unexpected};
            return Err(A::Error::invalid_value(Unexpected::Other(&key), &self));
        }
        let value = map.next_value()?; // send to Self::visit_seq
        Ok(value)
    }
}

impl WithVisitor for BaseHolder {
    type Visitor = BaseHolderVisitor;
    fn visitor_new() -> BaseHolderVisitor {
        BaseHolderVisitor {}
    }
}

#[derive(Clone, Debug, PartialEq, ruststep_derive::Holder)]
#[holder(table = Table)]
#[holder(field = sub1)]
pub struct Sub1 {
    #[holder(use_place_holder)]
    base: Base,
    y1: f64,
}

impl<'de> de::Deserialize<'de> for Sub1Holder {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_tuple_struct("SUB_1", 2, Sub1HolderVisitor {})
    }
}

pub struct Sub1HolderVisitor;

impl<'de> ::serde::de::Visitor<'de> for Sub1HolderVisitor {
    type Value = Sub1Holder;
    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(formatter, "SUB_1")
    }

    fn visit_seq<A>(self, mut seq: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: ::serde::de::SeqAccess<'de>,
    {
        if let Some(size) = seq.size_hint() {
            if size != 2 {
                use ::serde::de::Error;
                return Err(A::Error::invalid_length(size, &self));
            }
        }
        let base = seq.next_element()?.unwrap();
        let y1 = seq.next_element()?.unwrap();
        Ok(Sub1Holder { base, y1 })
    }

    // Entry point for Record or Parameter::Typed
    fn visit_map<A>(self, mut map: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: ::serde::de::MapAccess<'de>,
    {
        let key: String = map
            .next_key()?
            .expect("Empty map cannot be accepted as ruststep Holder"); // this must be a bug, not runtime error
        if key != "SUB_1" {
            use ::serde::de::{Error, Unexpected};
            return Err(A::Error::invalid_value(Unexpected::Other(&key), &self));
        }
        let value = map.next_value()?; // send to Self::visit_seq
        Ok(value)
    }
}

impl WithVisitor for Sub1Holder {
    type Visitor = Sub1HolderVisitor;
    fn visitor_new() -> Sub1HolderVisitor {
        Sub1HolderVisitor {}
    }
}

#[derive(Clone, Debug, PartialEq, ruststep_derive::Holder)]
#[holder(table = Table)]
#[holder(field = sub2)]
pub struct Sub2 {
    #[holder(use_place_holder)]
    base: Base,
    y2: f64,
}

impl<'de> de::Deserialize<'de> for Sub2Holder {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_tuple_struct("SUB_2", 2, Sub2HolderVisitor {})
    }
}

pub struct Sub2HolderVisitor;

impl<'de> ::serde::de::Visitor<'de> for Sub2HolderVisitor {
    type Value = Sub2Holder;
    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(formatter, "SUB_2")
    }

    fn visit_seq<A>(self, mut seq: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: ::serde::de::SeqAccess<'de>,
    {
        if let Some(size) = seq.size_hint() {
            if size != 2 {
                use ::serde::de::Error;
                return Err(A::Error::invalid_length(size, &self));
            }
        }
        let base = seq.next_element()?.unwrap();
        let y2 = seq.next_element()?.unwrap();
        Ok(Sub2Holder { base, y2 })
    }

    // Entry point for Record or Parameter::Typed
    fn visit_map<A>(self, mut map: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: ::serde::de::MapAccess<'de>,
    {
        let key: String = map
            .next_key()?
            .expect("Empty map cannot be accepted as ruststep Holder"); // this must be a bug, not runtime error
        if key != "SUB_2" {
            use ::serde::de::{Error, Unexpected};
            return Err(A::Error::invalid_value(Unexpected::Other(&key), &self));
        }
        let value = map.next_value()?; // send to Self::visit_seq
        Ok(value)
    }
}

impl WithVisitor for Sub2Holder {
    type Visitor = Sub2HolderVisitor;
    fn visitor_new() -> Sub2HolderVisitor {
        Sub2HolderVisitor {}
    }
}

#[derive(Clone, Debug, PartialEq, ruststep_derive::Holder)]
#[holder(table = Table)]
enum BaseAny {
    #[holder(field = base)]
    Base(Box<Base>),
    #[holder(field = sub1)]
    Sub1(Box<Sub1>),
    #[holder(field = sub2)]
    Sub2(Box<Sub2>),
}

impl EntityTable<as_holder!(BaseAny)> for Table {
    fn get_owned(&self, _entity_id: u64) -> Result<BaseAny> {
        todo!()
    }
    fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<BaseAny>> + 'table> {
        todo!()
    }
}

#[test]
fn deserialize_base() {
    let (residual, p): (_, Record) = exchange::simple_record("BASE(1.0)").finish().unwrap();
    dbg!(&p);
    assert_eq!(residual, "");

    let a: BaseHolder = Deserialize::deserialize(&p).unwrap();
    dbg!(&a);
    assert_eq!(a, BaseHolder { x: 1.0 });
}

#[test]
fn deserialize_sub1() {
    test(
        "SUB_1(BASE((1.0)), 2.0)",
        Sub1Holder {
            base: BaseHolder { x: 1.0 }.into(),
            y1: 2.0,
        },
    );
    test(
        "SUB_1(#3, 2.0)",
        Sub1Holder {
            base: RValue::Entity(3).into(),
            y1: 2.0,
        },
    );

    fn test(input: &str, answer: Sub1Holder) {
        let (residual, p): (_, Record) = exchange::simple_record(input).finish().unwrap();
        dbg!(&p);
        assert_eq!(residual, "");

        let a: Sub1Holder = Deserialize::deserialize(&p).unwrap();
        dbg!(&a);
        assert_eq!(a, answer);
    }
}

#[test]
fn deserialize_base_any() {
    test(
        "SUB_1(BASE((1.0)), 2.0)",
        BaseAnyHolder::Sub1(Box::new(Sub1Holder {
            base: BaseHolder { x: 1.0 }.into(),
            y1: 2.0,
        })),
    );
    test(
        "SUB_1(#3, 2.0)",
        BaseAnyHolder::Sub1(Box::new(Sub1Holder {
            base: RValue::Entity(3).into(),
            y1: 2.0,
        })),
    );

    fn test(input: &str, answer: BaseAnyHolder) {
        let (residual, p): (_, Record) = exchange::simple_record(input).finish().unwrap();
        dbg!(&p);
        assert_eq!(residual, "");

        let a: BaseAnyHolder = Deserialize::deserialize(&p).unwrap();
        dbg!(&a);
        assert_eq!(a, answer);
    }
}

#[test]
fn deserialize_base_any_placeholder() {
    test(
        "SUB_1(BASE((1.0)), 2.0)",
        PlaceHolder::Owned(BaseAnyHolder::Sub1(Box::new(Sub1Holder {
            base: BaseHolder { x: 1.0 }.into(),
            y1: 2.0,
        }))),
    );
    test(
        "SUB_1(#3, 2.0)",
        PlaceHolder::Owned(BaseAnyHolder::Sub1(Box::new(Sub1Holder {
            base: RValue::Entity(3).into(),
            y1: 2.0,
        }))),
    );

    fn test(input: &str, answer: PlaceHolder<BaseAnyHolder>) {
        let (residual, p): (_, Record) = exchange::simple_record(input).finish().unwrap();
        dbg!(&p);
        assert_eq!(residual, "");

        let a: PlaceHolder<BaseAnyHolder> = Deserialize::deserialize(&p).unwrap();
        dbg!(&a);
        assert_eq!(a, answer);
    }
}

#[test]
fn into_base_any() {
    test(
        "SUB_1(BASE((1.0)), 2.0)",
        BaseAny::Sub1(Box::new(Sub1 {
            base: Base { x: 1.0 },
            y1: 2.0,
        })),
    );
    test(
        "SUB_1(#1, 2.0)",
        BaseAny::Sub1(Box::new(Sub1 {
            base: Base { x: 1.0 },
            y1: 2.0,
        })),
    );

    fn test(input: &str, answer: BaseAny) {
        let table = Table::example();

        let (residual, p): (_, Record) = exchange::simple_record(input).finish().unwrap();
        dbg!(&p);
        assert_eq!(residual, "");

        let holder = PlaceHolder::<BaseAnyHolder>::deserialize(&p).unwrap();
        dbg!(&holder);

        let owned = holder.into_owned(&table).unwrap();
        dbg!(&owned);
        assert_eq!(owned, answer);
    }
}

#[test]
fn lookup_base_any() {
    test(
        Parameter::RValue(RValue::Entity(1)),
        BaseAny::Base(Box::new(Base { x: 1.0 })),
    );
    test(
        Parameter::RValue(RValue::Entity(2)),
        BaseAny::Sub1(Box::new(Sub1 {
            base: Base { x: 1.0 },
            y1: 2.0,
        })),
    );
    test(
        Parameter::RValue(RValue::Entity(2)),
        BaseAny::Sub2(Box::new(Sub2 {
            base: Base { x: 1.0 },
            y2: 2.0,
        })),
    );

    fn test(p: Parameter, answer: BaseAny) {
        let table = Table::example();

        let holder = PlaceHolder::<BaseAnyHolder>::deserialize(&p).unwrap();
        dbg!(&holder);

        let owned = holder.into_owned(&table).unwrap();
        dbg!(&owned);
        assert_eq!(owned, answer);
    }
}
