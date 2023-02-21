use std::{
    borrow::Cow,
    collections::{HashMap, VecDeque},
    marker::PhantomData,
};

use super::{QueryError, Result};

type QueryMap<'a> = HashMap<Cow<'a, str>, Value<'a>>;
type QueryVec<'a> = Vec<(Cow<'a, str>, Value<'a>)>;

enum Value<'a> {
    Missing,
    Single(Cow<'a, str>),
    Many(VecDeque<Cow<'a, str>>),
}

struct Deserializer<'de> {
    kvs: &'de mut QueryVec<'de>,
}

impl<'de> Deserializer<'de> {
    fn new(map: &'de mut QueryVec<'de>) -> Self {
        Self { kvs: map }
    }
}

macro_rules! fail_not_supported {
    ($($name:ident),*) => {
        $(
            fn $name<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                Err(serde::de::Error::custom("deserialize not supported"))
            }
        )*
    };
}

impl<'a, 'de> serde::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = QueryError;

    fail_not_supported!(
        deserialize_any,
        deserialize_bool,
        deserialize_i8,
        deserialize_i16,
        deserialize_i32,
        deserialize_i64,
        deserialize_u8,
        deserialize_u16,
        deserialize_u32,
        deserialize_u64,
        deserialize_f32,
        deserialize_f64,
        deserialize_char,
        deserialize_str,
        deserialize_string,
        deserialize_bytes,
        deserialize_byte_buf,
        deserialize_option,
        deserialize_unit
    );

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(serde::de::Error::custom(
            "deserialize_unit_struct not supported",
        ))
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(serde::de::Error::custom(
            "deserialize_newtype_struct not supported",
        ))
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(serde::de::Error::custom("deserialize_seq not supported"))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(serde::de::Error::custom("deserialize_tuple not supported"))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(serde::de::Error::custom(
            "deserialize_tuple_struct not supported",
        ))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(serde::de::Error::custom("deserialize_enum not supported"))
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(serde::de::Error::custom(
            "deserialize_identifier not supported",
        ))
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(serde::de::Error::custom(
            "deserialize_ignored_any not supported",
        ))
    }
}

impl<'a, 'de> serde::de::MapAccess<'de> for &'a mut Deserializer<'de> {
    type Error = QueryError;

    fn next_key_seed<K>(&mut self, seed: K) -> std::result::Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        match self.kvs.last() {
            Some((key, _)) => Ok(Some(serde::de::DeserializeSeed::deserialize(
                seed,
                CowDeserializer::new(Some(key.clone())),
            )?)),
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.kvs.pop() {
            Some((_, value)) => {
                let mut deserializer = ValueDeserializer::new(value);
                Ok(serde::de::DeserializeSeed::deserialize(
                    seed,
                    &mut deserializer,
                )?)
            }
            None => Err(serde::de::Error::custom(
                "next_value_seed called with no more values",
            )),
        }
    }
}

struct ValueDeserializer<'de> {
    value: Option<Value<'de>>,
}

impl<'de> ValueDeserializer<'de> {
    fn new(value: Value<'de>) -> Self {
        Self { value: Some(value) }
    }
}

impl<'a, 'de> serde::Deserializer<'de> for &'a mut ValueDeserializer<'de> {
    type Error = QueryError;

    fn deserialize_any<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value.take() {
            Some(Value::Missing) => CowDeserializer::new(None).deserialize_any(visitor),
            Some(Value::Single(c)) => CowDeserializer::new(Some(c)).deserialize_any(visitor),
            Some(v @ Value::Many(_)) => visitor.visit_seq(&mut ValueDeserializer::new(v)),
            None => Err(serde::de::Error::custom("deserialize called with no value")),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value.take() {
            Some(Value::Missing) => CowDeserializer::new(None).deserialize_u32(visitor),
            Some(Value::Single(c)) => CowDeserializer::new(Some(c)).deserialize_u32(visitor),
            Some(v @ Value::Many(_)) => visitor.visit_seq(&mut ValueDeserializer::new(v)),
            None => Err(serde::de::Error::custom("deserialize called with no value")),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value.take() {
            Some(Value::Missing) => visitor.visit_some(CowDeserializer::new(None)),
            Some(Value::Single(c)) => visitor.visit_some(CowDeserializer::new(Some(c))),
            Some(v @ Value::Many(_)) => visitor.visit_seq(&mut ValueDeserializer::new(v)),
            None => visitor.visit_none(),
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u64 f32 f64 char str string bytes byte_buf unit
        unit_struct tuple map struct enum identifier ignored_any
    }

    fn deserialize_seq<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(self)
    }
}

impl<'a, 'de> serde::de::SeqAccess<'de> for &'a mut ValueDeserializer<'de> {
    type Error = QueryError;

    fn next_element_seed<T>(
        &mut self,
        seed: T,
    ) -> std::result::Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        let (result, clear_value) = match self.value.as_mut() {
            Some(Value::Missing) => {
                let result =
                    serde::de::DeserializeSeed::deserialize(seed, CowDeserializer::new(None))
                        .map(Some);
                (result, true)
            }
            Some(Value::Single(c)) => {
                let result = serde::de::DeserializeSeed::deserialize(
                    seed,
                    CowDeserializer::new(Some(c.clone())),
                )
                .map(Some);
                (result, true)
            }
            Some(Value::Many(v)) => match v.pop_front() {
                Some(c) => {
                    let result = serde::de::DeserializeSeed::deserialize(
                        seed,
                        CowDeserializer::new(Some(c.clone())),
                    )
                    .map(Some);
                    (result, v.is_empty())
                }
                None => (
                    Err(serde::de::Error::custom(
                        "next_element_seed called with no more values",
                    )),
                    false,
                ),
            },
            None => (Ok(None), false),
        };

        if clear_value {
            self.value = None;
        }

        result
    }
}

struct CowDeserializer<'a, 'de> {
    cow: Option<Cow<'de, str>>,
    _phatom: PhantomData<&'a ()>,
}

impl<'a, 'de> CowDeserializer<'a, 'de> {
    fn new(cow: Option<Cow<'de, str>>) -> Self {
        Self {
            cow,
            _phatom: PhantomData,
        }
    }

    fn parse<T>(self) -> Result<T, QueryError>
    where
        T: std::str::FromStr,
    {
        match self.cow {
            Some(cow) => match cow.parse() {
                Ok(t) => Ok(t),
                Err(_) => Err(serde::de::Error::custom("failed to parse")),
            },
            None => Err(serde::de::Error::custom("failed to parse, no value found")),
        }
    }
}

macro_rules! forward_to_parse {
    ($(($de:ident, $vi:ident)),*) => {
        $(
            fn $de<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
            where
                V: serde::de::Visitor<'de>,
            {
                self.parse().and_then(|t| visitor.$vi(t))
            }
        )*
    };
}

impl<'a, 'de> serde::Deserializer<'de> for CowDeserializer<'a, 'de> {
    type Error = QueryError;

    fn deserialize_any<V>(mut self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.cow.take() {
            Some(cow) => match cow {
                Cow::Borrowed(b) => visitor.visit_borrowed_str(b),
                Cow::Owned(o) => visitor.visit_string(o),
            },
            None => visitor.visit_none(),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.cow {
            Some(string) => match string.as_ref() {
                "true" | "1" => visitor.visit_bool(true),
                "false" | "0" => visitor.visit_bool(false),
                _ => Err(serde::de::Error::custom(format!(
                    "invalid boolean: '{string}'"
                ))),
            },
            // There was a key present, but no value. This is equivalent to a boolean value of true.
            None => visitor.visit_bool(true),
        }
    }

    forward_to_parse!(
        (deserialize_i8, visit_i8),
        (deserialize_i16, visit_i16),
        (deserialize_i32, visit_i32),
        (deserialize_i64, visit_i64),
        (deserialize_u8, visit_u8),
        (deserialize_u16, visit_u16),
        (deserialize_u32, visit_u32),
        (deserialize_u64, visit_u64),
        (deserialize_f32, visit_f32),
        (deserialize_f64, visit_f64)
    );

    fn deserialize_char<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(serde::de::Error::custom("bytes not supported"))
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(serde::de::Error::custom("byte_buf not supported"))
    }

    fn deserialize_option<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // If we are deserializing a Cow that means there was a key present
        Err(serde::de::Error::custom("option not supported"))
    }

    fn deserialize_unit<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(serde::de::Error::custom("seq not supported"))
    }

    fn deserialize_tuple<V>(
        self,
        _len: usize,
        _visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(serde::de::Error::custom("tuple not supported"))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(serde::de::Error::custom("tuple_struct not supported"))
    }

    fn deserialize_map<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(serde::de::Error::custom("map not supported"))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(serde::de::Error::custom("struct not supported"))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

fn query_to_query_map(query: &str) -> Result<QueryMap> {
    let mut map = QueryMap::default();

    for comp in query.split('&') {
        let (key, value) = match comp.rsplit_once('=') {
            Some((key, value)) => {
                let key = percent_encoding::percent_decode_str(key).decode_utf8_lossy();
                let value =
                    Value::Single(percent_encoding::percent_decode_str(value).decode_utf8_lossy());
                (key, value)
            }
            None => {
                let key = percent_encoding::percent_decode_str(comp).decode_utf8_lossy();
                let value = Value::Missing;
                (key, value)
            }
        };

        match map.get_mut(&key) {
            Some(v) if std::matches!(v, Value::Single(_)) => match value {
                Value::Missing => {
                    return Err(serde::de::Error::custom(
                        "found key-only kv when a kv already exists",
                    ))
                }
                Value::Single(s) => {
                    let prev = match v {
                        Value::Single(s) => s.clone(),
                        _ => unreachable!(),
                    };
                    *v = Value::Many(VecDeque::from(vec![prev, s]));
                }
                Value::Many(_) => unreachable!(),
            },
            Some(Value::Many(values)) => match value {
                Value::Missing => {
                    return Err(serde::de::Error::custom(
                        "found key-only kv when a kv already exists",
                    ))
                }
                Value::Single(s) => {
                    values.push_back(s);
                }
                Value::Many(_) => unreachable!(),
            },
            Some(Value::Missing) => {
                match value {
                    Value::Missing => {} // Ignore duplicate key-only kv
                    Value::Single(_) => {
                        return Err(serde::de::Error::custom(
                            "found kv when a key-only kv already exists",
                        ))
                    }
                    Value::Many(_) => unreachable!(),
                }
            }
            None => {
                map.insert(key, value);
            }
            // Workaround for the first match arm since we actually cover all cases
            _ => unreachable!(),
        }
    }

    Ok(map)
}

pub fn from_query<T>(query: &str) -> Result<T, QueryError>
where
    T: serde::de::DeserializeOwned,
{
    let map = query_to_query_map(query)?;
    let mut vec = map.into_iter().collect();
    let mut deserializer = Deserializer::new(&mut vec);
    let value = serde::de::Deserialize::deserialize(&mut deserializer)?;
    Ok(value)
}
