use crate::{error::*, step::*};
use serde::{de, forward_to_deserialize_any, ser};

/// A struct typed in EXPRESS schema
///
/// serde::Deserialize
/// -------------------
///
/// ```
/// use nom::Finish;
/// use serde::Deserialize;
/// use ruststep::parser::exchange;
///
/// #[derive(Debug, Deserialize)]
/// struct MyStruct {
///     x: f64,
///     y: f64,
/// }
///
/// let (_, record) = exchange::simple_record("MY_STRUCT(1.0, 2.0)").finish().unwrap();
/// let a: MyStruct = Deserialize::deserialize(&record).unwrap();
/// ```
///
#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub name: String,
    pub parameters: Vec<Parameter>,
}

impl<'de, 'record> de::Deserializer<'de> for &'record Record {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(de::Error::invalid_type(
            de::Unexpected::Other("any"),
            &self.name.as_str(),
        ))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let seq = de::value::SeqDeserializer::new(self.parameters.iter());
        visitor.visit_seq(seq)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map enum identifier ignored_any
    }
}

impl<'record> ser::Serializer for &'record mut Record {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        todo!()
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        todo!()
    }
    fn serialize_i16(self, v: i16) -> Result<()> {
        todo!()
    }
    fn serialize_i32(self, v: i32) -> Result<()> {
        todo!()
    }
    fn serialize_i64(self, v: i64) -> Result<()> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        todo!()
    }
    fn serialize_u16(self, v: u16) -> Result<()> {
        todo!()
    }
    fn serialize_u32(self, v: u32) -> Result<()> {
        todo!()
    }
    fn serialize_u64(self, v: u64) -> Result<()> {
        todo!()
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        todo!()
    }
    fn serialize_f64(self, v: f64) -> Result<()> {
        todo!()
    }

    fn serialize_char(self, v: char) -> Result<()> {
        todo!()
    }
    fn serialize_str(self, v: &str) -> Result<()> {
        todo!()
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        todo!()
    }

    fn serialize_none(self) -> Result<()> {
        todo!()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<()> {
        todo!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        todo!()
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        todo!()
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        todo!()
    }
}

impl<'record> ser::SerializeSeq for &'record mut Record {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }
    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'record> ser::SerializeTuple for &'record mut Record {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'record> ser::SerializeTupleStruct for &'record mut Record {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'record> ser::SerializeTupleVariant for &'record mut Record {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'record> ser::SerializeMap for &'record mut Record {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'record> ser::SerializeStruct for &'record mut Record {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'record> ser::SerializeStructVariant for &'record mut Record {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}
