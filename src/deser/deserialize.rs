use std::{borrow::Cow, fmt, marker::PhantomData};

pub use serde::de::Error;
use serde::de::Unexpected;

use crate::common::Version;

use super::Format;

pub trait Deserializer<'de>: Sized {
    type Error: Error;
    type Native: serde::Deserializer<'de, Error = Self::Error>;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn format(&self) -> Format;

    fn version(&self) -> Version;

    fn into_serde(self) -> Self::Native;
}

pub trait Visitor<'de>: Sized {
    type Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result;

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(
            Unexpected::Bool(v),
            &ExpectedWrapper(&self),
        ))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(
            Unexpected::Signed(v),
            &ExpectedWrapper(&self),
        ))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_u64(v as u64)
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_u64(v as u64)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_u64(v as u64)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(
            Unexpected::Unsigned(v),
            &ExpectedWrapper(&self),
        ))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_f64(v as f64)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(
            Unexpected::Float(v),
            &ExpectedWrapper(&self),
        ))
    }

    #[inline]
    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let mut buf = [0u8; 4];
        self.visit_str(v.encode_utf8(&mut buf))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(
            Unexpected::Str(v),
            &ExpectedWrapper(&self),
        ))
    }

    #[inline]
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(v)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(&v)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let _ = v;
        Err(Error::invalid_type(
            Unexpected::Bytes(v),
            &ExpectedWrapper(&self),
        ))
    }

    #[inline]
    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_bytes(v)
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_bytes(&v)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(
            Unexpected::Option,
            &ExpectedWrapper(&self),
        ))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let _ = deserializer;
        Err(Error::invalid_type(
            Unexpected::Option,
            &ExpectedWrapper(&self),
        ))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(
            Unexpected::Unit,
            &ExpectedWrapper(&self),
        ))
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let _ = deserializer;
        Err(Error::invalid_type(
            Unexpected::NewtypeStruct,
            &ExpectedWrapper(&self),
        ))
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let _ = seq;
        Err(Error::invalid_type(
            Unexpected::Seq,
            &ExpectedWrapper(&self),
        ))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let _ = map;
        Err(Error::invalid_type(
            Unexpected::Map,
            &ExpectedWrapper(&self),
        ))
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        let _ = data;
        Err(Error::invalid_type(
            Unexpected::Enum,
            &ExpectedWrapper(&self),
        ))
    }
}

pub trait SeqAccess<'de> {
    type Error: Error;

    fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
    where
        T: Deserialize<'de>;

    fn size_hint(&self) -> Option<usize>;
}

pub trait MapAccess<'de> {
    type Error: Error;

    fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error>
    where
        K: Deserialize<'de>;

    fn next_value<V>(&mut self) -> Result<V, Self::Error>
    where
        V: Deserialize<'de>;

    fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error>
    where
        K: Deserialize<'de>,
        V: Deserialize<'de>;

    fn size_hint(&self) -> Option<usize>;
}

pub trait EnumAccess<'de>: Sized {
    type Error: Error;

    type Variant: VariantAccess<'de, Error = Self::Error>;

    fn variant<V>(self) -> Result<(V, Self::Variant), Self::Error>
    where
        V: Deserialize<'de>;
}

pub trait VariantAccess<'de>: Sized {
    type Error: Error;

    fn unit_variant(self) -> Result<(), Self::Error>;

    fn newtype_variant<T>(self) -> Result<T, Self::Error>
    where
        T: Deserialize<'de>;

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
}

pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}

struct ExpectedWrapper<'a, V>(&'a V);

impl<'de, 'a, V> serde::de::Expected for ExpectedWrapper<'a, V>
where
    V: Visitor<'de>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.expecting(f)
    }
}

pub struct SubsonicDeserializer<D> {
    format: Format,
    version: Version,
    deserializer: D,
}

impl<D> SubsonicDeserializer<D> {
    pub fn new(format: Format, version: Version, deserializer: D) -> Self {
        SubsonicDeserializer {
            format,
            version,
            deserializer,
        }
    }
}

impl<'de, D> Deserializer<'de> for SubsonicDeserializer<D>
where
    D: serde::Deserializer<'de>,
{
    type Error = D::Error;
    type Native = D;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_any(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_bool(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_i8(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_i16(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_i32(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_i64(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_u8(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_u16(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_u32(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_u64(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_f32(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_f64(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_char(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_str(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_string(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_bytes(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_byte_buf(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_option(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_unit(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_unit_struct(
            name,
            SubsonicVisitor {
                format: self.format,
                version: self.version,
                visitor,
            },
        )
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_newtype_struct(
            name,
            SubsonicVisitor {
                format: self.format,
                version: self.version,
                visitor,
            },
        )
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_seq(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_tuple(
            len,
            SubsonicVisitor {
                format: self.format,
                version: self.version,
                visitor,
            },
        )
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_tuple_struct(
            name,
            len,
            SubsonicVisitor {
                format: self.format,
                version: self.version,
                visitor,
            },
        )
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_map(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_struct(
            name,
            fields,
            SubsonicVisitor {
                format: self.format,
                version: self.version,
                visitor,
            },
        )
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_enum(
            name,
            variants,
            SubsonicVisitor {
                format: self.format,
                version: self.version,
                visitor,
            },
        )
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_identifier(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_ignored_any(SubsonicVisitor {
            format: self.format,
            version: self.version,
            visitor,
        })
    }

    fn format(&self) -> Format {
        self.format
    }

    fn version(&self) -> Version {
        self.version
    }

    fn into_serde(self) -> Self::Native {
        self.deserializer
    }
}

struct SubsonicVisitor<V> {
    format: Format,
    version: Version,
    visitor: V,
}

impl<'de, V> serde::de::Visitor<'de> for SubsonicVisitor<V>
where
    V: Visitor<'de>,
{
    type Value = V::Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.visitor.expecting(formatter)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visitor.visit_bool(v)
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visitor.visit_i8(v)
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visitor.visit_i16(v)
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visitor.visit_i32(v)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visitor.visit_i64(v)
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visitor.visit_u8(v)
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visitor.visit_u16(v)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visitor.visit_u32(v)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visitor.visit_u64(v)
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visitor.visit_f32(v)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visitor.visit_f64(v)
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visitor.visit_char(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visitor.visit_str(v)
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visitor.visit_borrowed_str(v)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visitor.visit_string(v)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visitor.visit_bytes(v)
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visitor.visit_borrowed_bytes(v)
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visitor.visit_byte_buf(v)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visitor.visit_none()
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        self.visitor.visit_some(SubsonicDeserializer {
            format: self.format,
            version: self.version,
            deserializer,
        })
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visitor.visit_unit()
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        self.visitor.visit_newtype_struct(SubsonicDeserializer {
            format: self.format,
            version: self.version,
            deserializer,
        })
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        self.visitor.visit_seq(SubsonicSeqAccess {
            format: self.format,
            version: self.version,
            access: seq,
        })
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        self.visitor.visit_map(SubsonicMapAccess {
            format: self.format,
            version: self.version,
            access: map,
        })
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        let _ = data;
        Err(Error::invalid_type(Unexpected::Enum, &self))
    }
}

struct SubsonicSeed<T> {
    format: Format,
    version: Version,
    _phantom: PhantomData<T>,
}

impl<T> SubsonicSeed<T> {
    fn new(format: Format, version: Version) -> Self {
        Self {
            format,
            version,
            _phantom: PhantomData,
        }
    }
}

impl<'de, T> serde::de::DeserializeSeed<'de> for SubsonicSeed<T>
where
    T: Deserialize<'de>,
{
    type Value = T;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(SubsonicDeserializer {
            format: self.format,
            version: self.version,
            deserializer,
        })
    }
}

struct SubsonicSeqAccess<A> {
    format: Format,
    version: Version,
    access: A,
}

impl<'de, A> SeqAccess<'de> for SubsonicSeqAccess<A>
where
    A: serde::de::SeqAccess<'de>,
{
    type Error = A::Error;

    fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
    where
        T: Deserialize<'de>,
    {
        self.access
            .next_element_seed(SubsonicSeed::new(self.format, self.version))
    }

    fn size_hint(&self) -> Option<usize> {
        self.access.size_hint()
    }
}

struct SubsonicMapAccess<A> {
    format: Format,
    version: Version,
    access: A,
}

impl<'de, A> MapAccess<'de> for SubsonicMapAccess<A>
where
    A: serde::de::MapAccess<'de>,
{
    type Error = A::Error;

    fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error>
    where
        K: Deserialize<'de>,
    {
        self.access
            .next_key_seed(SubsonicSeed::new(self.format, self.version))
    }

    fn next_value<V>(&mut self) -> Result<V, Self::Error>
    where
        V: Deserialize<'de>,
    {
        self.access
            .next_value_seed(SubsonicSeed::new(self.format, self.version))
    }

    fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error>
    where
        K: Deserialize<'de>,
        V: Deserialize<'de>,
    {
        self.access.next_entry_seed(
            SubsonicSeed::new(self.format, self.version),
            SubsonicSeed::new(self.format, self.version),
        )
    }

    fn size_hint(&self) -> Option<usize> {
        self.access.size_hint()
    }
}

struct SubsonicEnumAccess<A> {
    format: Format,
    version: Version,
    access: A,
}

impl<'de, A> EnumAccess<'de> for SubsonicEnumAccess<A>
where
    A: serde::de::EnumAccess<'de>,
{
    type Error = A::Error;

    type Variant = SubsonicVariantAccess<A::Variant>;

    fn variant<V>(self) -> Result<(V, Self::Variant), Self::Error>
    where
        V: Deserialize<'de>,
    {
        let (v, a) = self
            .access
            .variant_seed(SubsonicSeed::new(self.format, self.version))?;
        Ok((
            v,
            SubsonicVariantAccess {
                format: self.format,
                version: self.version,
                access: a,
            },
        ))
    }
}

struct SubsonicVariantAccess<A> {
    format: Format,
    version: Version,
    access: A,
}

impl<'de, A> VariantAccess<'de> for SubsonicVariantAccess<A>
where
    A: serde::de::VariantAccess<'de>,
{
    type Error = A::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        self.access.unit_variant()
    }

    fn newtype_variant<T>(self) -> Result<T, Self::Error>
    where
        T: Deserialize<'de>,
    {
        self.access
            .newtype_variant_seed(SubsonicSeed::new(self.format, self.version))
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.access.tuple_variant(
            len,
            SubsonicVisitor {
                format: self.format,
                version: self.version,
                visitor,
            },
        )
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.access.struct_variant(
            fields,
            SubsonicVisitor {
                format: self.format,
                version: self.version,
                visitor,
            },
        )
    }
}

macro_rules! impl_deserialize {
    ($($t:ty),*) => {
        $(
            impl<'de> Deserialize<'de> for $t {
                fn deserialize<D>(
                    deserializer: D,
                ) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>
                {
                    <$t as serde::Deserialize>::deserialize(deserializer.into_serde())
                }
            }
        )*
    };
}

impl_deserialize!(
    bool,
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
    char,
    String,
    (),
    serde::de::IgnoredAny,
    serde_value::Value
);

impl<'de, T> Deserialize<'de> for Option<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for V<T>
        where
            T: Deserialize<'de>,
        {
            type Value = T;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an optional value")
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                T::deserialize(deserializer)
            }
        }

        deserializer.deserialize_option(V(PhantomData))
    }
}

impl<'de, T> Deserialize<'de> for Vec<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for V<T>
        where
            T: Deserialize<'de>,
        {
            type Value = Vec<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut values = Vec::new();

                while let Some(value) = seq.next_element()? {
                    values.push(value);
                }

                Ok(values)
            }
        }

        deserializer.deserialize_seq(V(PhantomData))
    }
}

impl<'de> Deserialize<'de> for Cow<'de, str> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Cow<'de, str>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Cow::Borrowed(v))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Cow::Owned(v.to_owned()))
            }
        }

        deserializer.deserialize_str(V)
    }
}
