// Test for deserializing Holder structs
//
// This tests does not use proc-macro in ruststep-derive in order not to depend on them.

use ruststep::{ast::*, error::*, parser::exchange, place_holder::PlaceHolder, tables::*};

use nom::Finish;
use serde::{de, Deserialize};
use std::{collections::HashMap, str::FromStr};

#[derive(Default)]
struct Table {
    a: HashMap<u64, AHolder>,
    b: HashMap<u64, BHolder>,
}

impl FromStr for Table {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        let data_sec = DataSection::from_str(input)?;
        Ok(Self::from_data_section(&data_sec)?)
    }
}

impl Table {
    fn example() -> Self {
        Self::from_str(
            r#"
            DATA;
              #1 = A(1.0, 2.0);
              #2 = B(3.0, A((4.0, 5.0)));
              #3 = B(6.0, #1);
            ENDSEC;
            "#,
        )
        .unwrap()
    }

    fn append_data_section(&mut self, data_sec: &DataSection) -> Result<()> {
        for entity in &data_sec.entities {
            match entity {
                EntityInstance::Simple { id, record } => match record.name.as_str() {
                    "A" => {
                        if let Some(_) = self.a.insert(*id, Deserialize::deserialize(record)?) {
                            return Err(Error::DuplicatedEntity(*id));
                        }
                    }
                    "B" => {
                        if let Some(_) = self.b.insert(*id, Deserialize::deserialize(record)?) {
                            return Err(Error::DuplicatedEntity(*id));
                        }
                    }
                    _ => {
                        return Err(Error::UnknownEntityName {
                            entity_name: record.name.clone(),
                            schema: "test_holder".to_string(),
                        });
                    }
                },
                EntityInstance::Complex { .. } => {
                    unimplemented!("Complex entity is not supported")
                }
            }
        }
        Ok(())
    }

    fn from_data_section(data_sec: &DataSection) -> Result<Self> {
        let mut table = Self::default();
        table.append_data_section(data_sec)?;
        Ok(table)
    }
}

impl EntityTable<AHolder> for Table {
    fn get_owned(&self, entity_id: u64) -> Result<A> {
        get_owned(self, &self.a, entity_id)
    }
    fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<A>> + 'table> {
        owned_iter(self, &self.a)
    }
}

impl EntityTable<BHolder> for Table {
    fn get_owned(&self, entity_id: u64) -> Result<B> {
        get_owned(self, &self.b, entity_id)
    }
    fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<B>> + 'table> {
        owned_iter(self, &self.b)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct A {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct AHolder {
    x: f64,
    y: f64,
}

impl<'de> de::Deserialize<'de> for AHolder {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_tuple_struct("A", 2, AHolderVisitor {})
    }
}

struct AHolderVisitor;

impl<'de> ::serde::de::Visitor<'de> for AHolderVisitor {
    type Value = AHolder;
    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(formatter, "A")
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
        let x = seq.next_element()?.unwrap();
        let y = seq.next_element()?.unwrap();
        Ok(AHolder { x, y })
    }

    // Entry point for Record or Parameter::Typed
    fn visit_map<A>(self, mut map: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: ::serde::de::MapAccess<'de>,
    {
        let key: String = map
            .next_key()?
            .expect("Empty map cannot be accepted as ruststep Holder"); // this must be a bug, not runtime error
        if key != "A" {
            use ::serde::de::{Error, Unexpected};
            return Err(A::Error::invalid_value(Unexpected::Other(&key), &self));
        }
        let value = map.next_value()?; // send to Self::visit_seq
        Ok(value)
    }
}

impl Holder for AHolder {
    type Owned = A;
    type Table = Table;
    fn into_owned(self, _table: &Self::Table) -> Result<Self::Owned> {
        let AHolder { x, y } = self;
        Ok(A { x, y })
    }
    fn name() -> &'static str {
        "A"
    }
    fn attr_len() -> usize {
        2
    }
}

impl WithVisitor for AHolder {
    type Visitor = AHolderVisitor;
    fn visitor_new() -> Self::Visitor {
        AHolderVisitor {}
    }
}

#[derive(Debug, Clone, PartialEq)]
struct B {
    z: f64,
    a: A,
}

#[derive(Debug, Clone, PartialEq)]
struct BHolder {
    z: f64,
    a: PlaceHolder<AHolder>,
}

impl<'de> de::Deserialize<'de> for BHolder {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_tuple_struct("A", 2, BHolderVisitor {})
    }
}

struct BHolderVisitor;

impl<'de> ::serde::de::Visitor<'de> for BHolderVisitor {
    type Value = BHolder;
    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(formatter, "A")
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
        let z = seq.next_element()?.unwrap();
        let a = seq.next_element()?.unwrap();
        Ok(BHolder { z, a })
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

impl Holder for BHolder {
    type Owned = B;
    type Table = Table;
    fn into_owned(self, table: &Self::Table) -> Result<Self::Owned> {
        let BHolder { z, a } = self;
        Ok(B {
            z,
            a: a.into_owned(table)?,
        })
    }
    fn name() -> &'static str {
        "B"
    }
    fn attr_len() -> usize {
        2
    }
}

impl WithVisitor for BHolder {
    type Visitor = BHolderVisitor;
    fn visitor_new() -> Self::Visitor {
        BHolderVisitor {}
    }
}

#[test]
fn deserialize_a_holder() {
    // from Record
    let (residual, p): (_, Record) = exchange::simple_record("A(1.0, 2.0)").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let a: AHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(a, AHolder { x: 1.0, y: 2.0 });

    // from Parameter::Typed
    let (residual, p): (_, Parameter) = exchange::parameter("A((1.0, 2.0))").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let a: AHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(a, AHolder { x: 1.0, y: 2.0 });

    // Test for PlaceHolder<AHolder>
    let (residual, p): (_, Record) = exchange::simple_record("A(1.0, 2.0)").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let a: PlaceHolder<AHolder> = Deserialize::deserialize(&p).unwrap();
    assert_eq!(a, PlaceHolder::Owned(AHolder { x: 1.0, y: 2.0 }));
}

#[test]
fn deserialize_b_holder_record() {
    // from Record
    let (residual, p): (_, Record) = exchange::simple_record("B(1.0, A((2.0, 3.0)))")
        .finish()
        .unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let b: BHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(
        b,
        BHolder {
            z: 1.0,
            a: PlaceHolder::Owned(AHolder { x: 2.0, y: 3.0 })
        }
    );
}

#[test]
fn deserialize_b_holder_record_ref() {
    // from Record with ref
    let (residual, p): (_, Record) = exchange::simple_record("B(1.0, #2)").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let b: BHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(
        b,
        BHolder {
            z: 1.0,
            a: PlaceHolder::Ref(RValue::Entity(2))
        }
    );
}

#[test]
fn deserialize_b_holder_parameter() {
    // from Parameter::Typed
    let (residual, p): (_, Parameter) = exchange::parameter("B((1.0, A((2.0, 3.0))))")
        .finish()
        .unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let b: BHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(
        b,
        BHolder {
            z: 1.0,
            a: PlaceHolder::Owned(AHolder { x: 2.0, y: 3.0 })
        }
    );
}

#[test]
fn deserialize_b_holder_parameter_ref() {
    // from Parameter::Typed with Ref
    let (residual, p): (_, Parameter) = exchange::parameter("B((1.0, #2))").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let b: BHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(
        b,
        BHolder {
            z: 1.0,
            a: PlaceHolder::Ref(RValue::Entity(2))
        }
    );
}

#[test]
fn get_owned_a() {
    let table = Table::example();
    let a = EntityTable::<AHolder>::get_owned(&table, 1).unwrap();
    assert_eq!(a, A { x: 1.0, y: 2.0 });
}

#[test]
fn get_owned_b2() {
    let table = Table::example();
    let b = EntityTable::<BHolder>::get_owned(&table, 2).unwrap();
    assert_eq!(
        b,
        B {
            z: 3.0,
            a: A { x: 4.0, y: 5.0 }
        }
    );
}

#[test]
fn get_owned_b3() {
    let table = Table::example();
    let b = EntityTable::<BHolder>::get_owned(&table, 3).unwrap();
    assert_eq!(
        b,
        B {
            z: 6.0,
            a: A { x: 1.0, y: 2.0 }
        }
    );
}
