use ruststep::{ast::Record, parser::exchange, place_holder::PlaceHolder};

use nom::Finish;
use serde::{de, Deserialize};

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

#[derive(Debug, Clone)]
struct B {
    z: f64,
    a: A,
}

#[derive(Debug, Clone)]
struct BHolder {
    z: f64,
    a: PlaceHolder<AHolder>,
}

#[test]
fn deserialize_a_holder_from_record() {
    let (residual, p): (_, Record) = exchange::simple_record("A(1.0, 2.0)").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let a: AHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(a, AHolder { x: 1.0, y: 2.0 });
}
