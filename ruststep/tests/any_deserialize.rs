use nom::Finish;
use ruststep::{ast::*, error::*, parser::exchange, place_holder::*, tables::*};
use ruststep_derive::as_holder;
use serde::{de, Deserialize};
use std::collections::HashMap;

pub struct Table {
    base: HashMap<u64, as_holder!(Base)>,
    sub1: HashMap<u64, as_holder!(Sub1)>,
    sub2: HashMap<u64, as_holder!(Sub2)>,
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
        deserializer.deserialize_tuple_struct("A", 2, BaseHolderVisitor {})
    }
}

pub struct BaseHolderVisitor;

impl<'de> ::serde::de::Visitor<'de> for BaseHolderVisitor {
    type Value = BaseHolder;
    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(formatter, "Base")
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
        if key != "Base" {
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
        deserializer.deserialize_tuple_struct("A", 2, Sub1HolderVisitor {})
    }
}

pub struct Sub1HolderVisitor;

impl<'de> ::serde::de::Visitor<'de> for Sub1HolderVisitor {
    type Value = Sub1Holder;
    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(formatter, "Sub1")
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
        if key != "B" {
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
        deserializer.deserialize_tuple_struct("A", 2, Sub2HolderVisitor {})
    }
}

pub struct Sub2HolderVisitor;

impl<'de> ::serde::de::Visitor<'de> for Sub2HolderVisitor {
    type Value = Sub2Holder;
    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(formatter, "Sub2")
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
        if key != "B" {
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
    #[holder(field = sub1)]
    Sub1(Box<Sub1>),
    #[holder(field = sub2)]
    Sub2(Box<Sub2>),
}

#[test]
fn deserialize_base_any() {
    let (residual, p): (_, Record) = exchange::simple_record("SUB_1(BASE((1.0)), 2.0)")
        .finish()
        .unwrap();
    dbg!(&p);
    assert_eq!(residual, "");

    let a: BaseAnyHolder = Deserialize::deserialize(&p).unwrap();
    dbg!(&a);
    assert_eq!(
        a,
        BaseAnyHolder::Sub1(Box::new(Sub1Holder {
            base: PlaceHolder::Owned(BaseHolder { x: 1.0 }),
            y1: 2.0
        }))
    );
}
