#[macro_use]
pub(crate) mod macros;
pub(crate) mod helper;

pub mod common;
pub mod request;
pub mod response;

mod wrapper;
use response::Response;
pub(crate) use subsonic_macro::SubsonicType;
pub use wrapper::{Json, Xml};

pub enum Format {
    Json,
    Xml,
}

pub trait SubsonicSerialize {
    fn serialize<S>(&self, serializer: S, format: Format) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;
}

pub trait SubsonicDeserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D, format: Format) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>;
}

pub trait SubsonicType<'de>: SubsonicSerialize + SubsonicDeserialize<'de> {}

impl<'de, T: SubsonicSerialize + SubsonicDeserialize<'de>> SubsonicType<'de> for T {}

impl<T> SubsonicSerialize for &T
where
    T: SubsonicSerialize,
{
    fn serialize<S>(&self, serializer: S, format: Format) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        T::serialize(*self, serializer, format)
    }
}

impl_subsonic_for_serde!(u8);
impl_subsonic_for_serde!(u16);
impl_subsonic_for_serde!(u32);
impl_subsonic_for_serde!(u64);
impl_subsonic_for_serde!(u128);
impl_subsonic_for_serde!(usize);
impl_subsonic_for_serde!(i8);
impl_subsonic_for_serde!(i16);
impl_subsonic_for_serde!(i32);
impl_subsonic_for_serde!(i64);
impl_subsonic_for_serde!(i128);
impl_subsonic_for_serde!(isize);
impl_subsonic_for_serde!(f32);
impl_subsonic_for_serde!(f64);
impl_subsonic_for_serde!(bool);
impl_subsonic_for_serde!(char);
impl_subsonic_for_serde!(String);

impl<T> SubsonicSerialize for Option<T>
where
    T: SubsonicSerialize,
{
    fn serialize<S>(&self, serializer: S, format: crate::Format) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Some(value) => value.serialize(serializer, format),
            None => serializer.serialize_none(),
        }
    }
}

impl<'de, T> SubsonicDeserialize<'de> for Option<T>
where
    T: SubsonicDeserialize<'de>,
{
    fn deserialize<D>(deserializer: D, format: crate::Format) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match format {
            Format::Json => {
                let value =
                    <Option<crate::Json<T>> as serde::Deserialize>::deserialize(deserializer)?;
                Ok(value.map(crate::Json::into_inner))
            }
            Format::Xml => {
                let value =
                    <Option<crate::Xml<T>> as serde::Deserialize>::deserialize(deserializer)?;
                Ok(value.map(crate::Xml::into_inner))
            }
        }
    }
}

impl<T> SubsonicSerialize for Vec<T>
where
    T: SubsonicSerialize,
{
    fn serialize<S>(&self, serializer: S, format: crate::Format) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match format {
            Format::Json => {
                let value: &Vec<crate::Json<T>> =
                    unsafe { std::mem::transmute::<&Vec<T>, &Vec<crate::Json<T>>>(self) };
                <Vec<crate::Json<T>> as serde::Serialize>::serialize(value, serializer)
            }
            Format::Xml => {
                let value: &Vec<crate::Xml<T>> =
                    unsafe { std::mem::transmute::<&Vec<T>, &Vec<crate::Xml<T>>>(self) };
                <Vec<crate::Xml<T>> as serde::Serialize>::serialize(value, serializer)
            }
        }
    }
}

impl<'de, T> SubsonicDeserialize<'de> for Vec<T>
where
    T: SubsonicDeserialize<'de>,
{
    fn deserialize<D>(deserializer: D, format: crate::Format) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match format {
            Format::Json => {
                let value = <Vec<crate::Json<T>> as serde::Deserialize>::deserialize(deserializer)?;
                Ok(value.into_iter().map(crate::Json::into_inner).collect())
            }
            Format::Xml => {
                let value = <Vec<crate::Xml<T>> as serde::Deserialize>::deserialize(deserializer)?;
                Ok(value.into_iter().map(crate::Xml::into_inner).collect())
            }
        }
    }
}

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
///         r#"{"subsonic_response":{"status":"ok","version":"1.16.1","license":{"valid":true}}}"#,
///         subsonic_types::to_json(&response)?
///     );
/// # Ok(())
/// # }
/// ```
pub fn to_json(response: &Response) -> Result<String, SerdeError> {
    use serde::Serialize;
    #[derive(Debug, Clone, PartialEq, Serialize)]
    struct SubsonicResponse<'a> {
        #[serde(rename = "subsonic-response")]
        pub subsonic_response: Json<&'a Response>,
    }
    let wrapper = Json::new(response);
    let response = SubsonicResponse {
        subsonic_response: wrapper,
    };
    Ok(serde_json::to_string(&response)?)
}

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
pub fn to_xml(response: &Response) -> Result<String, SerdeError> {
    use serde::Serialize;
    let wrapper = Xml::new(response);
    let mut buffer = String::default();
    let serializer = quick_xml::se::Serializer::with_root(&mut buffer, Some("subsonic-response"))?;
    wrapper.serialize(serializer)?;
    Ok(buffer)
}

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
///     let serialized = r#"{"subsonic_response":{"status":"ok","version":"1.16.1","license":{"valid":true}}}"#;
///     let deserialized = subsonic_types::from_json(serialized)?;
///     assert_eq!(
///         response,
///         deserialized
///     );
/// # Ok(())
/// # }
/// ```
pub fn from_json(json: &str) -> Result<Response, SerdeError> {
    use serde::Deserialize;

    /// XML attributes are deserialized as a map of key-value pairs.
    /// All values are strings but some structs require integers or floats.
    ///
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum ValueConversion {
        String,
        Integer,
        Float,
    }

    struct MapKeyDeserializeSeed<'de, S> {
        seed: S,
        _phantom: std::marker::PhantomData<&'de ()>,
    }

    impl<'de, S> MapKeyDeserializeSeed<'de, S> {
        fn new(seed: S) -> Self {
            Self {
                seed,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<'de, S> serde::de::DeserializeSeed<'de> for MapKeyDeserializeSeed<'de, S>
    where
        S: serde::de::DeserializeSeed<'de>,
    {
        type Value = S::Value;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            self.seed.deserialize(deserializer)
        }
    }

    struct MapAccess<'de, A> {
        map: A,
        _phantom: std::marker::PhantomData<&'de ()>,
    }

    impl<'de, A> MapAccess<'de, A> {
        fn new(map: A) -> Self {
            Self {
                map,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<'de, A> serde::de::MapAccess<'de> for MapAccess<'de, A>
    where
        A: serde::de::MapAccess<'de>,
    {
        type Error = A::Error;

        fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: serde::de::DeserializeSeed<'de>,
        {
            match self.map.next_key_seed(seed)? {
                Some(key) => todo!(),
                None => todo!(),
            }
        }

        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::DeserializeSeed<'de>,
        {
            todo!()
        }
    }

    struct MapVisitor<'de, V> {
        visitor: V,
        _phantom: std::marker::PhantomData<&'de ()>,
    }

    impl<'de, V> MapVisitor<'de, V> {
        fn new(visitor: V) -> Self {
            Self {
                visitor,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<'de, V> serde::de::Visitor<'de> for MapVisitor<'de, V>
    where
        V: serde::de::Visitor<'de>,
    {
        type Value = V::Value;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            self.visitor.expecting(formatter)
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let map = MapAccess::new(map);
            self.visitor.visit_map(map)
        }
    }

    struct RelaxedDeserializer<'de, D> {
        pub deserializer: D,
        pub _phantom: std::marker::PhantomData<&'de ()>,
    }

    impl<'de, D> serde::Deserializer<'de> for RelaxedDeserializer<'de, D>
    where
        D: serde::Deserializer<'de>,
    {
        type Error = D::Error;

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserializer.deserialize_any(visitor)
        }

        fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            struct BoolVisitor;

            impl<'de> serde::de::Visitor<'de> for BoolVisitor {
                type Value = bool;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a boolean")
                }

                fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(value)
                }

                fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(value != 0)
                }

                fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(value != 0)
                }

                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    match value {
                        "true" | "1" => Ok(true),
                        "false" | "0" => Ok(false),
                        _ => Err(E::invalid_value(
                            serde::de::Unexpected::Str(value),
                            &"a boolean",
                        )),
                    }
                }
            }

            visitor.visit_bool(self.deserializer.deserialize_any(BoolVisitor)?)
        }

        fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserialize_i64(visitor)
        }

        fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserialize_i64(visitor)
        }

        fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserialize_i64(visitor)
        }

        fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            todo!()
        }

        fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserialize_u64(visitor)
        }

        fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserialize_u64(visitor)
        }

        fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserialize_u64(visitor)
        }

        fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            struct U64Visitor;

            impl<'de> serde::de::Visitor<'de> for U64Visitor {
                type Value = u64;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a u64")
                }

                fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(value)
                }

                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    match value.parse() {
                        Ok(value) => Ok(value),
                        Err(_) => Err(E::invalid_value(
                            serde::de::Unexpected::Str(value),
                            &"a u64",
                        )),
                    }
                }
            }

            visitor.visit_u64(self.deserializer.deserialize_any(U64Visitor)?)
        }

        fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserialize_f64(visitor)
        }

        fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            struct F64Visitor;

            impl<'de> serde::de::Visitor<'de> for F64Visitor {
                type Value = f64;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a f64")
                }

                fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(value)
                }

                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    match value.parse() {
                        Ok(value) => Ok(value),
                        Err(_) => Err(E::invalid_value(
                            serde::de::Unexpected::Str(value),
                            &"a f64",
                        )),
                    }
                }
            }

            visitor.visit_f64(self.deserializer.deserialize_any(F64Visitor)?)
        }

        fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserializer.deserialize_char(visitor)
        }

        fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserializer.deserialize_str(visitor)
        }

        fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserializer.deserialize_any(visitor)
        }

        fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserializer.deserialize_bytes(visitor)
        }

        fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserializer.deserialize_byte_buf(visitor)
        }

        fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserializer.deserialize_option(visitor)
        }

        fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserializer.deserialize_unit(visitor)
        }

        fn deserialize_unit_struct<V>(
            self,
            _name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserialize_map(visitor)
        }

        fn deserialize_newtype_struct<V>(
            self,
            _name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserialize_map(visitor)
        }

        fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            todo!()
        }

        fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
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
            V: serde::de::Visitor<'de>,
        {
            self.deserialize_seq(visitor)
        }

        fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserializer.deserialize_map(MapVisitor::new(visitor))
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
            unimplemented!("you need to implement this")
        }

        fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserializer.deserialize_identifier(visitor)
        }

        fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            self.deserializer.deserialize_ignored_any(visitor)
        }
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    struct SubsonicResponse {
        #[serde(rename = "subsonic-response")]
        pub subsonic_response: Json<Response>,
    }

    #[derive(Debug)]
    struct RelaxedDeserializerWrapper(SubsonicResponse);
    impl<'de> Deserialize<'de> for RelaxedDeserializerWrapper {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let relaxed = RelaxedDeserializer {
                deserializer,
                _phantom: std::marker::PhantomData,
            };
            let value = SubsonicResponse::deserialize(relaxed)?;
            Ok(Self(value))
        }
    }

    let response: SubsonicResponse = serde_json::from_str(json)?;
    Ok(response.subsonic_response.into_inner())
}

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
pub fn from_xml(xml: &str) -> Result<Response, SerdeError> {
    let response: Xml<Response> = quick_xml::de::from_str(xml)?;
    Ok(response.into_inner())
}
