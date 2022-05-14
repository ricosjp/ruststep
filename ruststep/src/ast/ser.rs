use crate::{ast::*, error::*};
use serde::ser;
use std::convert::TryFrom;

/// Serialize struct into STEP [SimpleEntityInstance]
pub fn to_record(obj: &impl ser::Serialize) -> Result<SimpleEntityInstance> {
    let mut ser = RecordSerializer::default();
    obj.serialize(&mut ser)?;
    assert!(ser.stack.is_empty()); // should panic because this must be bug, not a valid input
    Ok(SimpleEntityInstance {
        name: ser.name,
        parameters: ser.parameters,
    })
}

#[derive(Default, Debug)]
struct RecordSerializer {
    name: String,
    parameters: Vec<Parameter>,
    // For supporting nested record e.g. `B(3.0, A((1.0, 2.0)))`
    stack: Vec<(String, Vec<Parameter>)>,
}

impl<'se> ser::Serializer for &'se mut RecordSerializer {
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
        if v {
            self.parameters
                .push(Parameter::Enumeration("TRUE".to_string()));
        } else {
            self.parameters
                .push(Parameter::Enumeration("FALSE".to_string()));
        }
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_i64(self, v: i64) -> Result<()> {
        self.parameters.push(Parameter::Integer(v));
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_u64(self, v: u64) -> Result<()> {
        self.serialize_i64(i64::try_from(v).expect("integer larger than i64::MAX is not supported"))
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }
    fn serialize_f64(self, v: f64) -> Result<()> {
        self.parameters.push(Parameter::Real(v));
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }
    fn serialize_str(self, v: &str) -> Result<()> {
        self.parameters.push(Parameter::String(v.to_string()));
        Ok(())
    }
    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        unimplemented!("Bytes is not supported yet")
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.parameters.push(Parameter::NotProvided);
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        unimplemented!("Newtype variant is not suuported yet.")
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!("Tuple variant is not suuported yet.")
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(self)
    }

    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        if self.name.is_empty() {
            self.name = name.to_string();
        } else {
            // Entering sub struct e.g.
            //
            // ```
            // B(3.0, A((1.0, 2.0)))
            //        ^ here
            // ```
            //
            // Put current parameters (`3.0` as above) onto the top of stack,
            // and start serializing `A((1.0, 2.0))`.
            // This stack will be popped in SerializeStruct::end()
            //
            let current_name = std::mem::replace(&mut self.name, name.to_string());
            let current_params = std::mem::take(&mut self.parameters);
            self.stack.push((current_name, current_params));
        }
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!("Struct variant is not suuported yet.")
    }
}

impl<'se> ser::SerializeSeq for &'se mut RecordSerializer {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'se> ser::SerializeTuple for &'se mut RecordSerializer {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'se> ser::SerializeTupleStruct for &'se mut RecordSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'se> ser::SerializeTupleVariant for &'se mut RecordSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'se> ser::SerializeMap for &'se mut RecordSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        unimplemented!("Serialize Map to SimpleEntityInstance is not supported yet.")
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        unimplemented!("Serialize Map to SimpleEntityInstance is not supported yet.")
    }

    fn end(self) -> Result<()> {
        unimplemented!("Serialize Map to SimpleEntityInstance is not supported yet.")
    }
}

impl<'se> ser::SerializeStruct for &'se mut RecordSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        if let Some((name, params)) = self.stack.pop() {
            // restore stacked state
            let name = std::mem::replace(&mut self.name, name);
            let params = std::mem::replace(&mut self.parameters, params);
            self.parameters.push(Parameter::Typed {
                keyword: name,
                parameter: Box::new(params.into_iter().collect()),
            });
        }
        Ok(())
    }
}

impl<'se> ser::SerializeStructVariant for &'se mut RecordSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}
