use std::{fmt::Write, ops::Range};

use super::{QueryError, Result};

#[derive(Debug, Default)]
struct QuerySerializer {
    // Number of key-value pairs that have been serialized in the current sequence.
    // We assume no nested sequences.
    sequence_count: usize,
    pending_value: bool,
    pending_equal: bool,
    last_key: Option<Range<usize>>,
    query: String,
}

impl QuerySerializer {
    /// Erase the last key if we finished serializing a sequence with no values
    fn erase_last_key(&mut self) -> Result<()> {
        if let Some(last_key) = self.last_key.clone() {
            self.query.truncate(last_key.start);
            // If the query is not empty after removing the last key
            // then we need to remove the trailing ampersand.
            if !self.query.is_empty() {
                self.query.pop();
            }
            self.last_key = None;
            self.pending_equal = false;
            self.pending_value = false;
            Ok(())
        } else {
            Err(serde::ser::Error::custom(
                "no last key while erasing last key",
            ))
        }
    }

    fn serialize_last_key(&mut self) -> Result<()> {
        debug_assert!(!self.pending_value);
        debug_assert!(!self.pending_equal);
        if let Some(last_key) = self.last_key.clone() {
            self.pending_value = true;
            self.pending_equal = true;
            self.query.push('&');
            self.query.extend_from_within(last_key);
            Ok(())
        } else {
            Err(serde::ser::Error::custom("no last key"))
        }
    }

    fn serialize_key<K>(&mut self, key: K) -> Result<()>
    where
        K: serde::Serialize,
    {
        debug_assert!(!self.pending_value);
        debug_assert!(!self.pending_equal);

        if !self.query.is_empty() {
            self.query.push('&');
        }

        self.pending_value = true;
        self.pending_equal = true;

        let start = self.query.len();
        serde::Serialize::serialize(&key, &mut *self)?;
        let end = self.query.len();
        self.last_key = Some(start..end);

        Ok(())
    }

    fn serialize_value<V>(&mut self, value: V) -> Result<()>
    where
        V: serde::Serialize,
    {
        debug_assert!(self.pending_value);
        if self.pending_equal {
            self.query.push('=');
            self.pending_equal = false;
        }
        serde::Serialize::serialize(&value, &mut *self)?;
        self.pending_value = false;
        Ok(())
    }

    fn serialize_seq_value<V>(&mut self, value: V) -> Result<()>
    where
        V: serde::Serialize,
    {
        if !self.pending_value {
            self.serialize_last_key()?;
        }
        self.serialize_value(value)
    }

    fn serialize_key_value<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: serde::Serialize,
        V: serde::Serialize,
    {
        debug_assert!(!self.pending_value);
        self.serialize_key(key)?;
        self.serialize_value(value)?;
        Ok(())
    }
}

macro_rules! forward_to_write {
    ($(($name:ident, $t:ty)),*) => {
        $(
            fn $name(self, v: $t) -> Result<Self::Ok, Self::Error> {
                write!(self.query, "{}", v).map_err(|e| QueryError::new(e.to_string()))
            }
        )*
    };
}

impl<'a> serde::Serializer for &'a mut QuerySerializer {
    type Ok = ();
    type Error = QueryError;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    forward_to_write!(
        (serialize_bool, bool),
        (serialize_i8, i8),
        (serialize_i16, i16),
        (serialize_i32, i32),
        (serialize_i64, i64),
        (serialize_u8, u8),
        (serialize_u16, u16),
        (serialize_u32, u32),
        (serialize_u64, u64),
        (serialize_f32, f32),
        (serialize_f64, f64)
    );

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.query
            .push_str(percent_encoding::percent_encode_byte(v as u8));
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.query.extend(percent_encoding::utf8_percent_encode(
            v,
            percent_encoding::NON_ALPHANUMERIC,
        ));
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.query.extend(percent_encoding::percent_encode(
            v,
            percent_encoding::NON_ALPHANUMERIC,
        ));
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.erase_last_key()?;
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(mut self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.sequence_count = 0;
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(self)
    }
}

impl<'a> serde::ser::SerializeSeq for &'a mut QuerySerializer {
    type Ok = ();
    type Error = QueryError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.sequence_count += 1;
        self.serialize_seq_value(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.sequence_count == 0 {
            self.erase_last_key()?;
        }
        Ok(())
    }
}

impl<'a> serde::ser::SerializeTuple for &'a mut QuerySerializer {
    type Ok = ();

    type Error = QueryError;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        Err(serde::ser::Error::custom("tuple not supported"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeTupleStruct for &'a mut QuerySerializer {
    type Ok = ();

    type Error = QueryError;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        Err(serde::ser::Error::custom("tuple struct not supported"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeTupleVariant for &'a mut QuerySerializer {
    type Ok = ();

    type Error = QueryError;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        Err(serde::ser::Error::custom("tuple variant not supported"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeMap for &'a mut QuerySerializer {
    type Ok = ();

    type Error = QueryError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        QuerySerializer::serialize_key(self, key)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        QuerySerializer::serialize_value(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeStruct for &'a mut QuerySerializer {
    type Ok = ();

    type Error = QueryError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.serialize_key_value(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeStructVariant for &'a mut QuerySerializer {
    type Ok = ();

    type Error = QueryError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        Err(serde::ser::Error::custom(
            "serialize struct variant not supported",
        ))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub fn to_query<T>(request: &T) -> String
where
    T: serde::Serialize,
{
    let mut serializer = QuerySerializer::default();
    match request.serialize(&mut serializer) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("error: {}", e);

            #[cfg(debug_assertions)]
            panic!("error: {}", e);
        }
    }
    serializer.query
}
