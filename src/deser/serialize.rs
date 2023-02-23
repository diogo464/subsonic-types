pub use serde::ser::Error;

use crate::common::Version;

use super::{Format, Impossible};

pub trait Serializer {
    type Ok;
    type Error: Error;
    type Native: serde::Serializer<Ok = Self::Ok, Error = Self::Error>;
    type SerializeSeq: SerializeSeq<Ok = Self::Ok, Error = Self::Error>;
    type SerializeTuple: SerializeTuple<Ok = Self::Ok, Error = Self::Error>;
    type SerializeTupleStruct: SerializeTupleStruct<Ok = Self::Ok, Error = Self::Error>;
    type SerializeTupleVariant: SerializeTupleVariant<Ok = Self::Ok, Error = Self::Error>;
    type SerializeMap: SerializeMap<Ok = Self::Ok, Error = Self::Error>;
    type SerializeStruct: SerializeStruct<Ok = Self::Ok, Error = Self::Error>;
    type SerializeStructVariant: SerializeStructVariant<Ok = Self::Ok, Error = Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error>;

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error>;

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error>;

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error>;

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error>;

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error>;

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error>;

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error>;

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error>;

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error>;

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error>;

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error>;

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error>;

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error>;

    fn serialize_none(self) -> Result<Self::Ok, Self::Error>;

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize;

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error>;

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error>;

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error>;

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize;

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize;

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error>;

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error>;

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error>;

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error>;

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error>;

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error>;

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error>;

    fn format(&self) -> Format;

    fn version(&self) -> Version;

    fn into_serde(self) -> Self::Native;
}

pub trait SerializeSeq {
    type Ok;
    type Error: Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize;

    fn end(self) -> Result<Self::Ok, Self::Error>;
}

pub trait SerializeTuple {
    type Ok;
    type Error: Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize;

    fn end(self) -> Result<Self::Ok, Self::Error>;
}

pub trait SerializeTupleStruct {
    type Ok;
    type Error: Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize;

    fn end(self) -> Result<Self::Ok, Self::Error>;
}

pub trait SerializeTupleVariant {
    type Ok;
    type Error: Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize;

    fn end(self) -> Result<Self::Ok, Self::Error>;
}

pub trait SerializeMap {
    type Ok;
    type Error: Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize;

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize;

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error>
    where
        K: Serialize,
        V: Serialize;

    fn end(self) -> Result<Self::Ok, Self::Error>;
}

pub trait SerializeStruct {
    type Ok;
    type Error: Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize;

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error>;

    fn end(self) -> Result<Self::Ok, Self::Error>;
}

pub trait SerializeStructVariant {
    type Ok;
    type Error: Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize;

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error>;

    fn end(self) -> Result<Self::Ok, Self::Error>;
}

pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

pub struct SubsonicSerializer<S> {
    format: Format,
    version: Version,
    serializer: S,
}

impl<S> SubsonicSerializer<S> {
    pub fn new(format: Format, version: Version, serializer: S) -> Self {
        Self {
            format,
            version,
            serializer,
        }
    }
}

impl<S> Serializer for SubsonicSerializer<S>
where
    S: serde::Serializer,
{
    type Ok = S::Ok;

    type Error = S::Error;

    type Native = S;

    type SerializeSeq = SubsonicSerializeSeq<S::SerializeSeq>;

    type SerializeTuple = SubsonicSerializeTuple<S::SerializeTuple>;

    type SerializeTupleStruct = SubsonicSerializeTupleStruct<S::SerializeTupleStruct>;

    type SerializeTupleVariant = SubsonicSerializeTupleVariant<S::SerializeTupleVariant>;

    type SerializeMap = SubsonicSerializeMap<S::SerializeMap>;

    type SerializeStruct = SubsonicSerializeStruct<S::SerializeStruct>;

    type SerializeStructVariant = SubsonicSerializeStructVariant<S::SerializeStructVariant>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_bool(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i64(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u64(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_f32(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_f64(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_char(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_str(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_bytes(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_none()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.serializer
            .serialize_some(&SubsonicWrapper(self.format, self.version, value))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serializer
            .serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.serializer
            .serialize_newtype_struct(name, &SubsonicWrapper(self.format, self.version, value))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.serializer.serialize_newtype_variant(
            name,
            variant_index,
            variant,
            &SubsonicWrapper(self.format, self.version, value),
        )
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.serializer
            .serialize_seq(len)
            .map(|serializer| SubsonicSerializeSeq {
                format: self.format,
                version: self.version,
                serializer,
            })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serializer
            .serialize_tuple(len)
            .map(|serializer| SubsonicSerializeTuple {
                format: self.format,
                version: self.version,
                serializer,
            })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serializer
            .serialize_tuple_struct(name, len)
            .map(|serializer| SubsonicSerializeTupleStruct {
                format: self.format,
                version: self.version,
                serializer,
            })
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serializer
            .serialize_tuple_variant(name, variant_index, variant, len)
            .map(|serializer| SubsonicSerializeTupleVariant {
                format: self.format,
                version: self.version,
                serializer,
            })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.serializer
            .serialize_map(len)
            .map(|serializer| SubsonicSerializeMap {
                format: self.format,
                version: self.version,
                serializer,
            })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serializer
            .serialize_struct(name, len)
            .map(|serializer| SubsonicSerializeStruct {
                format: self.format,
                version: self.version,
                serializer,
            })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serializer
            .serialize_struct_variant(name, variant_index, variant, len)
            .map(|serializer| SubsonicSerializeStructVariant {
                format: self.format,
                version: self.version,
                serializer,
            })
    }

    fn format(&self) -> Format {
        self.format
    }

    fn version(&self) -> Version {
        self.version
    }

    fn into_serde(self) -> Self::Native {
        self.serializer
    }
}

pub struct SubsonicSerializeSeq<S> {
    format: Format,
    version: Version,
    serializer: S,
}

impl<S> SerializeSeq for SubsonicSerializeSeq<S>
where
    S: serde::ser::SerializeSeq,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serializer
            .serialize_element(&SubsonicWrapper(self.format, self.version, value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.end()
    }
}

pub struct SubsonicSerializeTuple<S> {
    format: Format,
    version: Version,
    serializer: S,
}

impl<S> SerializeTuple for SubsonicSerializeTuple<S>
where
    S: serde::ser::SerializeTuple,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serializer
            .serialize_element(&SubsonicWrapper(self.format, self.version, value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.end()
    }
}

pub struct SubsonicSerializeTupleStruct<S> {
    format: Format,
    version: Version,
    serializer: S,
}

impl<S> SerializeTupleStruct for SubsonicSerializeTupleStruct<S>
where
    S: serde::ser::SerializeTupleStruct,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serializer
            .serialize_field(&SubsonicWrapper(self.format, self.version, value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.end()
    }
}

pub struct SubsonicSerializeTupleVariant<S> {
    format: Format,
    version: Version,
    serializer: S,
}

impl<S> SerializeTupleVariant for SubsonicSerializeTupleVariant<S>
where
    S: serde::ser::SerializeTupleVariant,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serializer
            .serialize_field(&SubsonicWrapper(self.format, self.version, value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.end()
    }
}

pub struct SubsonicSerializeMap<S> {
    format: Format,
    version: Version,
    serializer: S,
}

impl<S> serde::ser::SerializeMap for SubsonicSerializeMap<S>
where
    S: serde::ser::SerializeMap,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.serializer.serialize_key(key)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.serializer.serialize_value(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.end()
    }
}

impl<S> SerializeMap for SubsonicSerializeMap<S>
where
    S: serde::ser::SerializeMap,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serializer
            .serialize_key(&SubsonicWrapper(self.format, self.version, key))
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serializer
            .serialize_value(&SubsonicWrapper(self.format, self.version, value))
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error>
    where
        K: Serialize,
        V: Serialize,
    {
        self.serializer.serialize_entry(
            &SubsonicWrapper(self.format, self.version, key),
            &SubsonicWrapper(self.format, self.version, value),
        )
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.end()
    }
}

pub struct SubsonicSerializeStruct<S> {
    format: Format,
    version: Version,
    serializer: S,
}

impl<S> SerializeStruct for SubsonicSerializeStruct<S>
where
    S: serde::ser::SerializeStruct,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serializer
            .serialize_field(key, &SubsonicWrapper(self.format, self.version, value))
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.serializer.skip_field(key)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.end()
    }
}

pub struct SubsonicSerializeStructVariant<S> {
    format: Format,
    version: Version,
    serializer: S,
}

impl<S> SerializeStructVariant for SubsonicSerializeStructVariant<S>
where
    S: serde::ser::SerializeStructVariant,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serializer
            .serialize_field(key, &SubsonicWrapper(self.format, self.version, value))
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.serializer.skip_field(key)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.end()
    }
}

struct SubsonicWrapper<'a, T: ?Sized>(Format, Version, &'a T);

impl<'a, T: ?Sized> serde::Serialize for SubsonicWrapper<'a, T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        T::serialize(self.2, SubsonicSerializer::new(self.0, self.1, serializer))
    }
}

impl<Ok, Err> SerializeSeq for Impossible<Ok, Err>
where
    Err: Error,
{
    type Ok = Ok;
    type Error = Err;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Ok, Self::Error> {
        unreachable!()
    }
}

impl<Ok, Err> SerializeTuple for Impossible<Ok, Err>
where
    Err: Error,
{
    type Ok = Ok;
    type Error = Err;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Ok, Self::Error> {
        unreachable!()
    }
}

impl<Ok, Err> SerializeTupleStruct for Impossible<Ok, Err>
where
    Err: Error,
{
    type Ok = Ok;
    type Error = Err;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Ok, Self::Error> {
        unreachable!()
    }
}

impl<Ok, Err> SerializeTupleVariant for Impossible<Ok, Err>
where
    Err: Error,
{
    type Ok = Ok;
    type Error = Err;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Ok, Self::Error> {
        unreachable!()
    }
}

impl<Ok, Err> SerializeMap for Impossible<Ok, Err>
where
    Err: Error,
{
    type Ok = Ok;
    type Error = Err;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error>
    where
        K: Serialize,
        V: Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl<Ok, Err> SerializeStruct for Impossible<Ok, Err>
where
    Err: Error,
{
    type Ok = Ok;
    type Error = Err;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl<Ok, Err> SerializeStructVariant for Impossible<Ok, Err>
where
    Err: Error,
{
    type Ok = Ok;
    type Error = Err;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

pub struct FlatMapSerializer<'a, M> {
    format: Format,
    version: Version,
    map: &'a mut M,
}

impl<'a, M> FlatMapSerializer<'a, M> {
    pub fn new(format: Format, version: Version, map: &'a mut M) -> Self {
        Self {
            format,
            version,
            map,
        }
    }
}

impl<'a, M> FlatMapSerializer<'a, M>
where
    M: SerializeMap,
{
    fn unsupported<T>() -> Result<T, M::Error> {
        Err(Error::custom("flattening only supports maps"))
    }
}

impl<'a, M, E> Serializer for FlatMapSerializer<'a, M>
where
    E: Error,
    M: SerializeMap<Error = E> + serde::ser::SerializeMap<Error = E>,
{
    type Ok = ();
    type Error = E;
    type Native = serde::__private::ser::FlatMapSerializer<'a, SubsonicSerializeMap<M>>;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Self;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Self::unsupported()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Self::unsupported()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Self::unsupported()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Self::unsupported()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Self::unsupported()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Self::unsupported()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Self::unsupported()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Self::unsupported()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Self::unsupported()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Self::unsupported()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Self::unsupported()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Self::unsupported()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Self::unsupported()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Self::unsupported()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Self::unsupported()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Self::unsupported()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Self::unsupported()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Self::unsupported()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Self::unsupported()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Self::unsupported()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Self::unsupported()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Self::unsupported()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Self::unsupported()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Self::unsupported()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Self::unsupported()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Self::unsupported()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Self::unsupported()
    }

    fn format(&self) -> Format {
        self.format
    }

    fn version(&self) -> Version {
        self.version
    }

    fn into_serde(self) -> Self::Native {
        unimplemented!()
    }
}

impl<'a, M> SerializeMap for FlatMapSerializer<'a, M>
where
    M: SerializeMap,
{
    type Ok = ();

    type Error = M::Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error>
    where
        K: Serialize,
        V: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

macro_rules! impl_serialize {
    ($($t:ty),*) => {
        $(
            impl Serialize for $t {
                fn serialize<S>(
                    &self,
                    serializer: S,
                ) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    <$t as serde::Serialize>::serialize(self, serializer.into_serde())
                }
            }
        )*
    };
}

impl<T> Serialize for &T
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        T::serialize(self, serializer)
    }
}

impl_serialize!(
    i8,
    i16,
    i32,
    i64,
    u8,
    u16,
    u32,
    u64,
    f32,
    f64,
    bool,
    String,
    str,
    ()
);

impl<T> Serialize for Option<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Some(value) => T::serialize(value, serializer),
            None => serializer.serialize_none(),
        }
    }
}

impl<T> Serialize for Vec<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for element in self {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}
