use std::marker::PhantomData;

use serde::{
    de::{DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor},
    Deserializer,
};

use crate::common::Format;

use super::{Error, Value};

pub struct FlatMapDeserializer<'de> {
    format: Format,
    pairs: Vec<(Option<String>, Option<Value>)>,
    __phatom: PhantomData<&'de ()>,
}

impl FlatMapDeserializer<'_> {
    pub fn new(format: Format, pairs: Vec<(String, Value)>) -> Self {
        Self {
            format,
            pairs: pairs.into_iter().map(|(k, v)| (Some(k), Some(v))).collect(),
            __phatom: PhantomData,
        }
    }
}

impl<'de> Deserializer<'de> for FlatMapDeserializer<'de> {
    type Error = Error;

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str
        string bytes byte_buf unit unit_struct newtype_struct seq tuple
        tuple_struct identifier ignored_any
    }

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(serde::de::Error::custom(
            "can only deserialize map, enum or option",
        ))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.pairs.is_empty() {
            visitor.visit_none()
        } else {
            eprintln!("visit_some");
            visitor.visit_some(self)
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(FlatMapAccess::new(self.format, self.pairs))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        mut self,
        _name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        eprintln!("deserialize_enum");
        match self.pairs.pop() {
            Some((Some(k), Some(v))) if variants.contains(&k.as_str()) => {
                visitor.visit_enum(FlatEnumAccess::new(self.format, k, v))
            }
            _ => visitor.visit_unit(),
        }
    }
}

struct FlatMapAccess<'de> {
    format: Format,
    pairs: Vec<(Option<String>, Option<Value>)>,
    index: usize,
    __phatom: PhantomData<&'de ()>,
}

impl<'de> FlatMapAccess<'de> {
    fn new(format: Format, pairs: Vec<(Option<String>, Option<Value>)>) -> Self {
        Self {
            format,
            pairs,
            index: 0,
            __phatom: PhantomData,
        }
    }
}

impl<'de> MapAccess<'de> for FlatMapAccess<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        let key = match self.pairs.get_mut(self.index) {
            Some((key, _)) => match key.take() {
                Some(key) => key,
                None => return Err(serde::de::Error::custom("called next key twice in a row")),
            },
            None => return Ok(None),
        };
        Ok(Some(seed.deserialize(StringDeserializer::new(key))?))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let value = match self.pairs.get_mut(self.index) {
            Some((_, value)) => match value.take() {
                Some(value) => value,
                None => Err(serde::de::Error::custom(
                    "called next value without calling next key",
                ))?,
            },
            None => Err(serde::de::Error::custom(
                "called next value on empty map access",
            ))?,
        };
        self.index += 1;
        Ok(seed.deserialize(ValueDeserializer::new(self.format, value))?)
    }
}

struct StringDeserializer<'de> {
    string: String,
    __phantom: PhantomData<&'de ()>,
}

impl<'de> StringDeserializer<'de> {
    fn new(string: String) -> Self {
        Self {
            string,
            __phantom: PhantomData,
        }
    }
}

impl<'de> Deserializer<'de> for StringDeserializer<'de> {
    type Error = Error;

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str
        string bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct enum identifier ignored_any struct map
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.string)
    }
}

struct FlatSeqAccess<'de> {
    format: Format,
    values: Vec<Option<Value>>,
    index: usize,
    __phatom: PhantomData<&'de ()>,
}

impl<'de> FlatSeqAccess<'de> {
    fn new(format: Format, values: Vec<Value>) -> Self {
        Self {
            format,
            values: values.into_iter().map(Some).collect(),
            index: 0,
            __phatom: PhantomData,
        }
    }
}

impl<'de> SeqAccess<'de> for FlatSeqAccess<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.index >= self.values.len() {
            return Ok(None);
        }
        // SAFETY: Value must be Some because we always increment index after removing a value
        // and they all start as Some.
        let value = self.values[self.index].take().unwrap();
        self.index += 1;
        Ok(Some(
            seed.deserialize(ValueDeserializer::new(self.format, value))?,
        ))
    }
}

struct FlatEnumAccess<'de> {
    format: Format,
    key: String,
    value: Value,
    __phatom: PhantomData<&'de ()>,
}

impl<'de> FlatEnumAccess<'de> {
    fn new(format: Format, key: String, value: Value) -> Self {
        Self {
            format,
            key,
            value,
            __phatom: PhantomData,
        }
    }
}

impl<'de> EnumAccess<'de> for FlatEnumAccess<'de> {
    type Error = Error;
    type Variant = ValueDeserializer<'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        Ok((
            seed.deserialize(StringDeserializer::new(self.key))?,
            ValueDeserializer::new(self.format, self.value),
        ))
    }
}

struct ValueDeserializer<'de> {
    format: Format,
    value: Value,
    __phantom: PhantomData<&'de ()>,
}

impl<'de> ValueDeserializer<'de> {
    fn new(format: Format, value: Value) -> Self {
        Self {
            format,
            value,
            __phantom: PhantomData,
        }
    }
}

impl<'de> Deserializer<'de> for ValueDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Bool(v) => visitor.visit_bool(v),
            Value::String(v) => match v.as_str() {
                "true" | "1" => visitor.visit_bool(true),
                "false" | "0" => visitor.visit_bool(false),
                _ => Err(serde::de::Error::custom("invalid boolean")),
            },
            Value::Map(m) if self.format == Format::Xml => match m.into_iter().next() {
                Some((Value::String(key), Value::String(value)))
                    if key == "$value" || key == "$text" =>
                {
                    match value.as_str() {
                        "true" | "1" => visitor.visit_bool(true),
                        "false" | "0" => visitor.visit_bool(false),
                        _ => Err(serde::de::Error::custom("invalid boolean")),
                    }
                }
                _ => return Err(serde::de::Error::custom("expected f64")),
            },
            v => Err(serde::de::Error::custom(format!(
                "invalid boolean, found {:?}",
                v
            ))),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value: i64 = match self.value {
            Value::I8(v) => v as i64,
            Value::I16(v) => v as i64,
            Value::I32(v) => v as i64,
            Value::I64(v) => v,
            Value::U8(v) => v as i64,
            Value::U16(v) => v as i64,
            Value::U32(v) => v as i64,
            Value::U64(v) => i64::try_from(v).map_err(serde::de::Error::custom)?,
            Value::String(v) => v.parse().map_err(serde::de::Error::custom)?,
            Value::Map(m) => match m.into_iter().next() {
                Some((Value::String(key), Value::String(value)))
                    if key == "$value" || key == "$text" =>
                {
                    value.parse().map_err(serde::de::Error::custom)?
                }
                _ => Err(serde::de::Error::custom("expected signed integer"))?,
            },
            v => Err(serde::de::Error::custom(format!(
                "expected signed integer, found {v:#?}"
            )))?,
        };
        visitor.visit_i64(value)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = match self.value {
            Value::I8(v) if v >= 0 => v as u64,
            Value::I16(v) if v >= 0 => v as u64,
            Value::I32(v) if v >= 0 => v as u64,
            Value::I64(v) if v >= 0 => v as u64,
            Value::U8(v) => v as u64,
            Value::U16(v) => v as u64,
            Value::U32(v) => v as u64,
            Value::U64(v) => v,
            Value::String(v) if self.format == Format::Xml => {
                v.parse().map_err(serde::de::Error::custom)?
            }
            Value::Map(m) if self.format == Format::Xml => match m.into_iter().next() {
                Some((Value::String(key), Value::String(value)))
                    if key == "$value" || key == "$text" =>
                {
                    value.parse().map_err(serde::de::Error::custom)?
                }
                _ => Err(serde::de::Error::custom("expected unsigned integer"))?,
            },
            v => Err(serde::de::Error::custom(format!(
                "expected unsigned integer, found {v:#?}"
            )))?,
        };
        visitor.visit_u64(value)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_f64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value = match self.value {
            Value::I8(v) => v as f64,
            Value::I16(v) => v as f64,
            Value::I32(v) => v as f64,
            Value::I64(v) => v as f64,
            Value::U8(v) => v as f64,
            Value::U16(v) => v as f64,
            Value::U32(v) => v as f64,
            Value::U64(v) => v as f64,
            Value::F32(v) => v as f64,
            Value::F64(v) => v,
            Value::String(v) if self.format == Format::Xml => {
                v.parse().map_err(serde::de::Error::custom)?
            }
            Value::Map(m) if self.format == Format::Xml => match m.into_iter().next() {
                Some((Value::String(key), Value::String(value)))
                    if key == "$value" || key == "$text" =>
                {
                    value.parse().map_err(serde::de::Error::custom)?
                }
                _ => return Err(serde::de::Error::custom("expected f64")),
            },
            v => {
                return Err(serde::de::Error::custom(format!(
                    "expected f64, found {v:#?}"
                )))
            }
        };
        visitor.visit_f64(value)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Char(c) => visitor.visit_char(c),
            _ => Err(serde::de::Error::custom("expected char")),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let string = match self.value {
            Value::Bool(v) if self.format == Format::Xml => v.to_string(),
            Value::U8(v) if self.format == Format::Xml => v.to_string(),
            Value::U16(v) if self.format == Format::Xml => v.to_string(),
            Value::U32(v) if self.format == Format::Xml => v.to_string(),
            Value::U64(v) if self.format == Format::Xml => v.to_string(),
            Value::I8(v) if self.format == Format::Xml => v.to_string(),
            Value::I16(v) if self.format == Format::Xml => v.to_string(),
            Value::I32(v) if self.format == Format::Xml => v.to_string(),
            Value::I64(v) if self.format == Format::Xml => v.to_string(),
            Value::F32(v) if self.format == Format::Xml => v.to_string(),
            Value::F64(v) if self.format == Format::Xml => v.to_string(),
            Value::Char(v) => v.to_string(),
            Value::String(v) => v,
            Value::Map(m) if self.format == Format::Xml => match m.into_iter().next() {
                Some((Value::String(key), Value::String(value)))
                    if key == "$value" || key == "$text" =>
                {
                    value
                }
                _ => return Err(serde::de::Error::custom("expected string")),
            },
            v => {
                return Err(serde::de::Error::custom(format!(
                    "expected string, found {v:#?}"
                )))
            }
        };
        visitor.visit_string(string)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(serde::de::Error::custom("cannot deserialize bytes"))
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(serde::de::Error::custom("cannot deserialize byte buffer"))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Seq(array) => visitor.visit_seq(FlatSeqAccess::new(self.format, array)),
            v => visitor.visit_seq(FlatSeqAccess::new(self.format, vec![Value::from(v)])),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Map(map) => {
                let mut pairs = Vec::with_capacity(map.len());
                for (k, v) in map {
                    let k = match k {
                        Value::String(string) => string,
                        _ => return Err(serde::de::Error::custom("invalid key type")),
                    };
                    pairs.push((Some(k), Some(v)));
                }
                visitor.visit_map(FlatMapAccess::new(self.format, pairs))
            }
            _ => Err(serde::de::Error::custom("expected map value")),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

impl<'de> VariantAccess<'de> for ValueDeserializer<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_struct("", fields, visitor)
    }
}
