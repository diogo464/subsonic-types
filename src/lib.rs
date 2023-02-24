#![feature(string_extend_from_within)]

pub(crate) mod obj;
pub(crate) mod query;

pub mod common;
pub mod request;
pub mod response;

use response::Response;

#[derive(Debug)]
pub struct SerdeError(Box<dyn std::error::Error>);

impl std::fmt::Display for SerdeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for SerdeError {}

impl From<serde_json::Error> for SerdeError {
    fn from(error: serde_json::Error) -> Self {
        Self(Box::new(error))
    }
}

impl From<quick_xml::DeError> for SerdeError {
    fn from(error: quick_xml::DeError) -> Self {
        Self(Box::new(error))
    }
}

/// Serialize a response to json
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     use subsonic_types::{common::Version, response::{Response, ResponseBody, License}};
///     let response = Response::ok(
///         Version::V1_16_1,
///         ResponseBody::License(License {
///             valid: true,
///             ..Default::default()
///         }),
///     );
///     assert_eq!(
///         r#"{"subsonic-response":{"status":"ok","version":"1.16.1","license":{"valid":true}}}"#,
///         subsonic_types::to_json(&response)?
///     );
/// # Ok(())
/// # }
/// ```
// pub fn to_json(response: &Response) -> Result<String, SerdeError> {
//     use deser::SubsonicSerialize;
//     use serde::Serialize;
//     #[derive(Serialize)]
//     struct SubsonicResponse<'a> {
//         #[serde(rename = "subsonic-response")]
//         pub subsonic_response: <Response as SubsonicSerialize<'a>>::Output,
//     }
//     let response_output = <Response as SubsonicSerialize>::prepare(
//         response,
//         deser::Format::Json,
//         common::Version::LATEST,
//     );
//     let response = SubsonicResponse {
//         subsonic_response: response_output,
//     };
//     Ok(serde_json::to_string(&response)?)
// }

/// Serialize a response to xml
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     use subsonic_types::{common::Version, response::{Response, ResponseBody, License}};
///     let response = Response::ok(
///         Version::V1_16_1,
///         ResponseBody::License(License {
///             valid: true,
///             ..Default::default()
///         }),
///     );
///     assert_eq!(
///         r#"<subsonic-response status="ok" version="1.16.1"><license valid="true"/></subsonic-response>"#,
///         subsonic_types::to_xml(&response)?
///     );
/// # Ok(())
/// # }
/// ```
const _: () = {};
// pub fn to_xml(response: &Response) -> Result<String, SerdeError> {
//     use deser::SubsonicSerialize;
//     use serde::Serialize;

//     let response_output = <Response as SubsonicSerialize>::prepare(
//         response,
//         deser::Format::Xml,
//         common::Version::LATEST,
//     );
//     let mut buffer = String::default();
//     let serializer = quick_xml::se::Serializer::with_root(&mut buffer, Some("subsonic-response"))?;
//     response_output.serialize(serializer)?;
//     Ok(buffer)
// }

/// Deserialize a response from json
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     use subsonic_types::{common::Version, response::{Response, ResponseBody, License}};
///     let response = Response::ok(
///         Version::V1_16_1,
///         ResponseBody::License(License {
///             valid: true,
///             ..Default::default()
///         }),
///     );
///     let serialized = r#"{"subsonic-response":{"status":"ok","version":"1.16.1","license":{"valid":true}}}"#;
///     let deserialized = subsonic_types::from_json(serialized)?;
///     assert_eq!(
///         response,
///         deserialized
///     );
/// # Ok(())
/// # }
/// ```
const _: () = {};
// pub fn from_json(json: &str) -> Result<Response, SerdeError> {
//     use deser::Json;
//     use serde::Deserialize;

//     #[derive(Deserialize)]
//     struct SubsonicResponse {
//         #[serde(rename = "subsonic-response")]
//         subsonic_response: Json<Response>,
//     }

//     let response: SubsonicResponse = serde_json::from_str(json)?;
//     Ok(response.subsonic_response.into_inner())
// }

/// Deserialize a response from xml
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     use subsonic_types::{common::Version, response::{Response, ResponseBody, License}};
///     let response = Response::ok(
///         Version::V1_16_1,
///         ResponseBody::License(License {
///             valid: true,
///             ..Default::default()
///         }),
///     );
///     let serialized = r#"<subsonic-response status="ok" version="1.16.1"><license valid="true"/></subsonic-response>"#;
///     let deserialized = subsonic_types::from_xml(serialized)?;
///     assert_eq!(
///         response,
///         deserialized
///     );
/// # Ok(())
/// # }
/// ```
const _: () = {};
// pub fn from_xml(xml: &str) -> Result<Response, SerdeError> {
//     use deser::Xml;

//     let response: Xml<Response> = quick_xml::de::from_str(xml)?;
//     Ok(response.into_inner())
// }

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use serde::{
        de::{DeserializeSeed, MapAccess, SeqAccess, Visitor},
        ser::SerializeMap,
        Deserialize, Deserializer, Serialize, Serializer,
    };

    use crate::{
        common::{Format, Version},
        response::Genre,
    };

    #[derive(Debug, Default, PartialEq)]
    pub struct Foo {
        server: String,
        genre: Genre,
    }

    #[derive(Debug, Default, PartialEq)]
    pub struct Flattened {
        server: String,
        genre: Genre,
    }

    // Serialization
    trait SubsonicSerialize {
        fn serialize<S>(
            &self,
            serializer: S,
            format: Format,
            version: Version,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer;
    }

    macro_rules! impl_subsonic_serialize {
        ($($t:ty),*) => {
            $(
                impl SubsonicSerialize for $t {
                    fn serialize<S>(
                        &self,
                        serializer: S,
                        _: Format,
                        _: Version,
                    ) -> Result<S::Ok, S::Error>
                    where
                        S: Serializer,
                    {
                        <$t as serde::Serialize>::serialize(self, serializer)
                    }
                }
            )*
        };
    }
    impl_subsonic_serialize!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, bool, String);

    impl<T> SubsonicSerialize for &T
    where
        T: SubsonicSerialize,
    {
        fn serialize<S>(
            &self,
            serializer: S,
            format: Format,
            version: Version,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            (*self).serialize(serializer, format, version)
        }
    }

    struct SubsonicSerializeWrapper<T>(T, Format, Version);
    impl<T> Serialize for SubsonicSerializeWrapper<T>
    where
        T: SubsonicSerialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.0.serialize(serializer, self.1, self.2)
        }
    }

    impl SubsonicSerialize for Genre {
        fn serialize<S>(
            &self,
            serializer: S,
            format: Format,
            version: Version,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let song_count_name = match format {
                Format::Json => "songCount",
                Format::Xml => "@songCount",
            };
            let album_count_name = match format {
                Format::Json => "albumCount",
                Format::Xml => "@albumCount",
            };
            let name_name = match format {
                Format::Json => "name",
                Format::Xml => "name",
            };

            let mut map = serializer.serialize_map(Some(3))?;
            if version > Version::V1_10_2 {
                map.serialize_entry(
                    song_count_name,
                    &SubsonicSerializeWrapper(&self.song_count, format, version),
                )?;
            }
            if version > Version::V1_10_2 {
                map.serialize_entry(
                    album_count_name,
                    &SubsonicSerializeWrapper(&self.album_count, format, version),
                )?;
            }
            map.serialize_entry(
                "name",
                &SubsonicSerializeWrapper(&self.name, format, version),
            )?;
            map.end()
        }
    }

    impl SubsonicSerialize for Foo {
        fn serialize<S>(
            &self,
            serializer: S,
            format: Format,
            version: Version,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let server_name = match format {
                Format::Json => "server",
                Format::Xml => "@server",
            };
            let genre_name = match format {
                Format::Json => "genre",
                Format::Xml => "genre",
            };

            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry(
                server_name,
                &SubsonicSerializeWrapper(&self.server, format, version),
            )?;
            map.serialize_entry(
                genre_name,
                &SubsonicSerializeWrapper(&self.genre, format, version),
            )?;
            map.end()
        }
    }

    impl SubsonicSerialize for Flattened {
        fn serialize<S>(
            &self,
            serializer: S,
            format: Format,
            version: Version,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let server_name = match format {
                Format::Json => "server",
                Format::Xml => "@server",
            };
            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry(
                server_name,
                &SubsonicSerializeWrapper(&self.server, format, version),
            )?;
            self.genre.serialize(
                serde::__private::ser::FlatMapSerializer(&mut map),
                format,
                version,
            )?;
            map.end()
        }
    }

    // Deserialization
    trait SubsonicDeserialize<'de>: Sized {
        type Seed: DeserializeSeed<'de, Value = Self> + From<(Format, Version)>;
    }

    struct AnySeed<T>(PhantomData<T>);
    impl<'de, T> DeserializeSeed<'de> for AnySeed<T>
    where
        T: Deserialize<'de>,
    {
        type Value = T;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            T::deserialize(deserializer)
        }
    }
    impl<T> From<(Format, Version)> for AnySeed<T> {
        fn from(_: (Format, Version)) -> Self {
            Self(PhantomData)
        }
    }

    #[derive(Debug)]
    struct Error(String);

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::error::Error for Error {}

    impl serde::de::Error for Error {
        fn custom<T>(msg: T) -> Self
        where
            T: std::fmt::Display,
        {
            Self(msg.to_string())
        }
    }

    struct FlatMapDeserializer<'de> {
        format: Format,
        pairs: Vec<(Option<String>, Option<serde_value::Value>)>,
        __phatom: PhantomData<&'de ()>,
    }

    impl FlatMapDeserializer<'_> {
        fn new(format: Format, pairs: Vec<(String, serde_value::Value)>) -> Self {
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
            string bytes byte_buf option unit unit_struct newtype_struct seq tuple
            tuple_struct enum identifier ignored_any
        }

        fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            Err(serde::de::Error::custom("can only deserialize map"))
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
    }

    struct FlatMapAccess<'de> {
        format: Format,
        pairs: Vec<(Option<String>, Option<serde_value::Value>)>,
        index: usize,
        __phatom: PhantomData<&'de ()>,
    }

    impl<'de> FlatMapAccess<'de> {
        fn new(format: Format, pairs: Vec<(Option<String>, Option<serde_value::Value>)>) -> Self {
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
        values: Vec<serde_value::Value>,
        __phatom: PhantomData<&'de ()>,
    }

    impl<'de> FlatSeqAccess<'de> {
        fn new(format: Format, values: Vec<serde_value::Value>) -> Self {
            Self {
                format,
                values,
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
            let value = match self.values.pop() {
                Some(value) => value,
                None => return Ok(None),
            };
            Ok(Some(
                seed.deserialize(ValueDeserializer::new(self.format, value))?,
            ))
        }
    }

    struct ValueDeserializer<'de> {
        format: Format,
        value: serde_value::Value,
        __phantom: PhantomData<&'de ()>,
    }

    impl<'de> ValueDeserializer<'de> {
        fn new(format: Format, value: serde_value::Value) -> Self {
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
                serde_value::Value::Bool(v) => visitor.visit_bool(v),
                serde_value::Value::String(v) => match v.as_str() {
                    "true" | "1" => visitor.visit_bool(true),
                    "false" | "0" => visitor.visit_bool(false),
                    _ => Err(serde::de::Error::custom("invalid boolean")),
                },
                serde_value::Value::Map(m) if self.format == Format::Xml => {
                    match m.into_iter().next() {
                        Some((
                            serde_value::Value::String(key),
                            serde_value::Value::String(value),
                        )) if key == "$value" || key == "$text" => match value.as_str() {
                            "true" | "1" => visitor.visit_bool(true),
                            "false" | "0" => visitor.visit_bool(false),
                            _ => Err(serde::de::Error::custom("invalid boolean")),
                        },
                        _ => return Err(serde::de::Error::custom("expected f64")),
                    }
                }
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
                serde_value::Value::I8(v) => v as i64,
                serde_value::Value::I16(v) => v as i64,
                serde_value::Value::I32(v) => v as i64,
                serde_value::Value::I64(v) => v,
                serde_value::Value::U8(v) => v as i64,
                serde_value::Value::U16(v) => v as i64,
                serde_value::Value::U32(v) => v as i64,
                serde_value::Value::U64(v) => i64::try_from(v).map_err(serde::de::Error::custom)?,
                serde_value::Value::String(v) => v.parse().map_err(serde::de::Error::custom)?,
                serde_value::Value::Map(m) => match m.into_iter().next() {
                    Some((serde_value::Value::String(key), serde_value::Value::String(value)))
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
                serde_value::Value::I8(v) if v >= 0 => v as u64,
                serde_value::Value::I16(v) if v >= 0 => v as u64,
                serde_value::Value::I32(v) if v >= 0 => v as u64,
                serde_value::Value::I64(v) if v >= 0 => v as u64,
                serde_value::Value::U8(v) => v as u64,
                serde_value::Value::U16(v) => v as u64,
                serde_value::Value::U32(v) => v as u64,
                serde_value::Value::U64(v) => v,
                serde_value::Value::String(v) if self.format == Format::Xml => {
                    v.parse().map_err(serde::de::Error::custom)?
                }
                serde_value::Value::Map(m) if self.format == Format::Xml => {
                    match m.into_iter().next() {
                        Some((
                            serde_value::Value::String(key),
                            serde_value::Value::String(value),
                        )) if key == "$value" || key == "$text" => {
                            value.parse().map_err(serde::de::Error::custom)?
                        }
                        _ => Err(serde::de::Error::custom("expected unsigned integer"))?,
                    }
                }
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
                serde_value::Value::I8(v) => v as f64,
                serde_value::Value::I16(v) => v as f64,
                serde_value::Value::I32(v) => v as f64,
                serde_value::Value::I64(v) => v as f64,
                serde_value::Value::U8(v) => v as f64,
                serde_value::Value::U16(v) => v as f64,
                serde_value::Value::U32(v) => v as f64,
                serde_value::Value::U64(v) => v as f64,
                serde_value::Value::F32(v) => v as f64,
                serde_value::Value::F64(v) => v,
                serde_value::Value::String(v) if self.format == Format::Xml => {
                    v.parse().map_err(serde::de::Error::custom)?
                }
                serde_value::Value::Map(m) if self.format == Format::Xml => {
                    match m.into_iter().next() {
                        Some((
                            serde_value::Value::String(key),
                            serde_value::Value::String(value),
                        )) if key == "$value" || key == "$text" => {
                            value.parse().map_err(serde::de::Error::custom)?
                        }
                        _ => return Err(serde::de::Error::custom("expected f64")),
                    }
                }
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
                serde_value::Value::Char(c) => visitor.visit_char(c),
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
                serde_value::Value::Bool(v) if self.format == Format::Xml => v.to_string(),
                serde_value::Value::U8(v) if self.format == Format::Xml => v.to_string(),
                serde_value::Value::U16(v) if self.format == Format::Xml => v.to_string(),
                serde_value::Value::U32(v) if self.format == Format::Xml => v.to_string(),
                serde_value::Value::U64(v) if self.format == Format::Xml => v.to_string(),
                serde_value::Value::I8(v) if self.format == Format::Xml => v.to_string(),
                serde_value::Value::I16(v) if self.format == Format::Xml => v.to_string(),
                serde_value::Value::I32(v) if self.format == Format::Xml => v.to_string(),
                serde_value::Value::I64(v) if self.format == Format::Xml => v.to_string(),
                serde_value::Value::F32(v) if self.format == Format::Xml => v.to_string(),
                serde_value::Value::F64(v) if self.format == Format::Xml => v.to_string(),
                serde_value::Value::Char(v) => v.to_string(),
                serde_value::Value::String(v) => v,
                serde_value::Value::Map(m) if self.format == Format::Xml => {
                    match m.into_iter().next() {
                        Some((
                            serde_value::Value::String(key),
                            serde_value::Value::String(value),
                        )) if key == "$value" || key == "$text" => value,
                        _ => return Err(serde::de::Error::custom("expected string")),
                    }
                }
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
                serde_value::Value::Seq(array) => {
                    visitor.visit_seq(FlatSeqAccess::new(self.format, array))
                }
                _ => Err(serde::de::Error::custom("expected array")),
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
                serde_value::Value::Map(map) => {
                    let mut pairs = Vec::with_capacity(map.len());
                    for (k, v) in map {
                        let k = match k {
                            serde_value::Value::String(string) => string,
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

    macro_rules! impl_subsonic_deserialize {
        ($($t:ty),*) => {
            $(
                impl<'de> SubsonicDeserialize<'de> for $t {
                    type Seed = AnySeed<$t>;
                }
            )*
        };
    }
    impl_subsonic_deserialize!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, bool, String);

    // Deserialize for Genre
    const _: () = {
        pub struct Seed(Format, Version);
        impl From<(Format, Version)> for Seed {
            fn from((format, version): (Format, Version)) -> Self {
                Self(format, version)
            }
        }
        impl<'de> Visitor<'de> for Seed {
            type Value = Genre;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a genre")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let format = self.0;
                let version = self.1;

                let song_count_key = match format {
                    Format::Json => "songCount",
                    Format::Xml => "@songCount",
                };
                let album_count_key = match format {
                    Format::Json => "albumCount",
                    Format::Xml => "@albumCount",
                };
                let name_key = match format {
                    Format::Json => "name",
                    Format::Xml => "name",
                };

                let mut song_count = None;
                let mut album_count = None;
                let mut name = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key {
                        k if k == song_count_key => {
                            song_count =
                                Some(map.next_value_seed(
                                    <<u32 as SubsonicDeserialize>::Seed as From<(
                                        Format,
                                        Version,
                                    )>>::from((
                                        format, version,
                                    )),
                                )?);
                        }
                        k if k == album_count_key => {
                            album_count =
                                Some(map.next_value_seed(
                                    <<u32 as SubsonicDeserialize>::Seed as From<(
                                        Format,
                                        Version,
                                    )>>::from((
                                        format, version,
                                    )),
                                )?);
                        }
                        k if k == name_key => {
                            name = Some(map.next_value_seed(
                                <<String as SubsonicDeserialize>::Seed as From<(
                                    Format,
                                    Version,
                                )>>::from((format, version)),
                            )?);
                        }
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                if song_count.is_none() && version < Version::V1_10_2 {
                    song_count = Some(Default::default());
                }
                if album_count.is_none() && version < Version::V1_10_2 {
                    album_count = Some(Default::default());
                }

                let song_count =
                    song_count.ok_or_else(|| serde::de::Error::missing_field(song_count_key))?;
                let album_count =
                    album_count.ok_or_else(|| serde::de::Error::missing_field(album_count_key))?;
                let name = name.ok_or_else(|| serde::de::Error::missing_field(name_key))?;

                Ok(Genre {
                    song_count,
                    album_count,
                    name,
                })
            }
        }
        impl<'de> DeserializeSeed<'de> for Seed {
            type Value = Genre;

            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_map(self)
            }
        }
        impl<'de> SubsonicDeserialize<'de> for Genre {
            type Seed = Seed;
        }
    };

    // Deserialize for Foo
    const _: () = {
        pub struct Seed(Format, Version);
        impl From<(Format, Version)> for Seed {
            fn from((format, version): (Format, Version)) -> Self {
                Self(format, version)
            }
        }
        impl<'de> Visitor<'de> for Seed {
            type Value = Foo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a foo")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let format = self.0;
                let version = self.1;

                let server_key = match format {
                    Format::Json => "server",
                    Format::Xml => "@server",
                };
                let genre_key = match format {
                    Format::Json => "genre",
                    Format::Xml => "genre",
                };

                let mut server = None;
                let mut genre = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key {
                        k if k == server_key => {
                            server = Some(map.next_value_seed(
                                <<String as SubsonicDeserialize>::Seed as From<(
                                    Format,
                                    Version,
                                )>>::from((format, version)),
                            )?);
                        }
                        k if k == genre_key => {
                            genre =
                                Some(map.next_value_seed(
                                    <<Genre as SubsonicDeserialize>::Seed as From<(
                                        Format,
                                        Version,
                                    )>>::from((
                                        format, version,
                                    )),
                                )?);
                        }
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                let server = server.ok_or_else(|| serde::de::Error::missing_field(server_key))?;
                let genre = genre.ok_or_else(|| serde::de::Error::missing_field(genre_key))?;

                Ok(Foo { server, genre })
            }
        }
        impl<'de> DeserializeSeed<'de> for Seed {
            type Value = Foo;

            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_map(self)
            }
        }
        impl<'de> SubsonicDeserialize<'de> for Foo {
            type Seed = Seed;
        }
    };

    // Deserialize for Flattened
    const _: () = {
        pub struct Seed(Format, Version);
        impl From<(Format, Version)> for Seed {
            fn from((format, version): (Format, Version)) -> Self {
                Self(format, version)
            }
        }
        impl<'de> Visitor<'de> for Seed {
            type Value = Flattened;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a flattened")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let format = self.0;
                let version = self.1;

                let server_key = match format {
                    Format::Json => "server",
                    Format::Xml => "@server",
                };

                let mut server = None;
                let mut genre = None;

                let mut buffered = Vec::new();
                while let Some(key) = map.next_key::<String>()? {
                    match key {
                        k if k == server_key => {
                            server = Some(map.next_value_seed(
                                <<String as SubsonicDeserialize>::Seed as From<(
                                    Format,
                                    Version,
                                )>>::from((format, version)),
                            )?);
                        }
                        _ => {
                            buffered.push((key, map.next_value::<serde_value::Value>()?));
                        }
                    }
                }

                genre = Some(
                    <<Genre as SubsonicDeserialize>::Seed as From<(Format, Version)>>::from((
                        format, version,
                    ))
                    .deserialize(FlatMapDeserializer::new(format, buffered))
                    .map_err(serde::de::Error::custom)?,
                );

                let server = server.ok_or_else(|| serde::de::Error::missing_field(server_key))?;
                let genre = genre.ok_or_else(|| serde::de::Error::missing_field("genre"))?;

                Ok(Flattened { server, genre })
            }
        }
        impl<'de> DeserializeSeed<'de> for Seed {
            type Value = Flattened;

            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_map(self)
            }
        }
        impl<'de> SubsonicDeserialize<'de> for Flattened {
            type Seed = Seed;
        }
    };

    #[test]
    fn main() {
        let genre = Genre {
            name: "genre".to_string(),
            song_count: 1,
            album_count: 2,
        };

        let foo = Foo {
            server: "server".to_string(),
            genre: genre.clone(),
        };

        let flattend = Flattened {
            server: "server".to_string(),
            genre,
        };

        let json = serialize(&foo, Format::Json, Version::V1_16_1);
        let xml = serialize(&foo, Format::Xml, Version::V1_16_1);

        eprintln!("json: {}", json);
        eprintln!("xml: {}", xml);

        let json_foo: Foo = deserialize(&json, Format::Json, Version::V1_16_1);
        let xml_foo: Foo = deserialize(&xml, Format::Xml, Version::V1_16_1);

        eprintln!("json_foo: {:?}", json_foo);
        eprintln!("xml_foo: {:?}", xml_foo);

        let json_flattend = serialize(&flattend, Format::Json, Version::V1_16_1);
        let xml_flattend = serialize(&flattend, Format::Xml, Version::V1_16_1);

        eprintln!("json_flattend: {}", json_flattend);
        eprintln!("xml_flattend: {}", xml_flattend);

        let json_flattend: Flattened = deserialize(&json_flattend, Format::Json, Version::V1_16_1);
        let xml_flattend: Flattened = deserialize(&xml_flattend, Format::Xml, Version::V1_16_1);

        eprintln!("2 json_flattend: {:?}", json_flattend);
        eprintln!("2 xml_flattend: {:?}", xml_flattend);
    }

    fn serialize<T>(value: &T, format: Format, version: Version) -> String
    where
        T: SubsonicSerialize,
    {
        match format {
            Format::Json => {
                serde_json::to_string_pretty(&SubsonicSerializeWrapper(value, format, version))
                    .unwrap()
            }
            Format::Xml => {
                let mut buf = String::new();
                let serializer =
                    quick_xml::se::Serializer::with_root(&mut buf, Some("subsonic-response"))
                        .unwrap();
                SubsonicSerializeWrapper(value, format, version)
                    .serialize(serializer)
                    .unwrap();
                buf
            }
        }
    }

    fn deserialize<T>(value: &str, format: Format, version: Version) -> T
    where
        T: for<'de> SubsonicDeserialize<'de>,
    {
        match format {
            Format::Json => {
                let mut de = serde_json::Deserializer::from_str(value);
                T::Seed::from((format, version))
                    .deserialize(&mut de)
                    .unwrap()
            }
            Format::Xml => {
                let mut de = quick_xml::de::Deserializer::from_str(value);
                T::Seed::from((format, version))
                    .deserialize(&mut de)
                    .unwrap()
            }
        }
    }
}
