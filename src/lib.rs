#![feature(string_extend_from_within)]

#[macro_use]
pub(crate) mod deser;
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
    use crate::common::Version;
    use crate::deser::{deserialize::*, serialize::*, Format};
    use crate::response::Genre;

    #[derive(Debug, PartialEq)]
    struct Foo {
        server: String,
        genre: Genre,
    }

    impl Serialize for Genre {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let format = serializer.format();
            let version = serializer.version();
            match format {
                crate::deser::Format::Json => {
                    let mut map = serializer.serialize_map(Some(3))?;
                    if version >= Version::V1_10_2 {
                        map.serialize_entry("songCount", &self.song_count)?;
                    }
                    if version >= Version::V1_10_2 {
                        map.serialize_entry("albumCount", &self.album_count)?;
                    }
                    map.serialize_entry("name", &self.name)?;
                    map.end()
                }
                crate::deser::Format::Xml => {
                    let mut map = serializer.serialize_map(Some(3))?;
                    if version >= Version::V1_10_2 {
                        map.serialize_entry("@songCount", &self.song_count)?;
                    }
                    if version >= Version::V1_10_2 {
                        map.serialize_entry("@albumCount", &self.album_count)?;
                    }
                    map.serialize_entry("$value", &self.name)?;
                    map.end()
                }
            }
        }
    }

    impl Serialize for Foo {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry("server", &self.server)?;
            map.serialize_entry("genre", &self.genre)?;
            map.end()
        }
    }

    impl<'de> Deserialize<'de> for Genre {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct V(Format, Version);
            impl<'de> Visitor<'de> for V {
                type Value = Genre;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a genre")
                }

                fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
                where
                    A: MapAccess<'de>,
                {
                    let format = self.0;
                    let version = self.1;
                    let mut map = map;

                    let mut name = None;
                    let mut song_count = None;
                    let mut album_count = None;

                    while let Some(key) = map.next_key::<String>()? {
                        match key.as_str() {
                            "name" if format == Format::Json => {
                                name = Some(map.next_value()?);
                            }
                            "$text" if format == Format::Xml => {
                                name = Some(map.next_value()?);
                            }
                            "songCount" if format == Format::Json => {
                                song_count = Some(map.next_value()?);
                            }
                            "albumCount" if format == Format::Json => {
                                album_count = Some(map.next_value()?);
                            }
                            "@songCount" if format == Format::Xml => {
                                song_count = Some(map.next_value()?);
                            }
                            "@albumCount" if format == Format::Xml => {
                                album_count = Some(map.next_value()?);
                            }
                            _ => {
                                eprintln!("unknown key: {}", key);
                                map.next_value::<serde::de::IgnoredAny>()?;
                            }
                        }
                    }

                    if version < Version::V1_10_2 {
                        song_count = Some(Default::default());
                    }
                    if version < Version::V1_10_2 {
                        album_count = Some(Default::default());
                    }

                    Ok(Genre {
                        name: name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                        song_count: song_count
                            .ok_or_else(|| serde::de::Error::missing_field("songCount"))?,
                        album_count: album_count
                            .ok_or_else(|| serde::de::Error::missing_field("albumCount"))?,
                    })
                }
            }

            let visitor = V(deserializer.format(), deserializer.version());
            deserializer.deserialize_map(visitor)
        }
    }

    impl<'de> Deserialize<'de> for Foo {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct V(Format, Version);
            impl<'de> Visitor<'de> for V {
                type Value = Foo;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a foo")
                }

                fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
                where
                    A: MapAccess<'de>,
                {
                    let format = self.0;
                    let version = self.1;
                    let mut map = map;

                    let mut server = None;
                    let mut genre = None;

                    while let Some(key) = map.next_key::<String>()? {
                        match key.as_str() {
                            "server" => {
                                server = Some(map.next_value()?);
                            }
                            "genre" => {
                                genre = Some(map.next_value()?);
                            }
                            _ => {
                                map.next_value::<serde::de::IgnoredAny>()?;
                            }
                        }
                    }

                    Ok(Foo {
                        server: server.ok_or_else(|| serde::de::Error::missing_field("server"))?,
                        genre: genre.ok_or_else(|| serde::de::Error::missing_field("genre"))?,
                    })
                }
            }

            let visitor = V(deserializer.format(), deserializer.version());
            deserializer.deserialize_map(visitor)
        }
    }

    #[test]
    #[should_panic]
    fn main() {
        let genre = Genre {
            name: "genre".to_string(),
            song_count: 1,
            album_count: 2,
        };

        let foo = Foo {
            server: "server".to_string(),
            genre,
        };

        let json0 = serialize(Format::Json, Version::V1_9_0, &foo);
        let json2 = serialize(Format::Json, Version::V1_10_2, &foo);
        let xml0 = serialize(Format::Xml, Version::V1_9_0, &foo);
        let xml2 = serialize(Format::Xml, Version::V1_10_2, &foo);

        eprintln!("json0: {}", json0);
        eprintln!("json2: {}", json2);
        eprintln!("xml0: {}", xml0);
        eprintln!("xml2: {}", xml2);

        let foo0 = deserialize::<Foo>(Format::Json, Version::V1_9_0, &json0);
        let foo2 = deserialize::<Foo>(Format::Json, Version::V1_10_2, &json2);
        let foo3 = deserialize::<Foo>(Format::Xml, Version::V1_9_0, &xml0);
        let foo4 = deserialize::<Foo>(Format::Xml, Version::V1_10_2, &xml2);

        eprintln!("foo0: {:?}", foo0);
        eprintln!("foo2: {:?}", foo2);
        eprintln!("foo3: {:?}", foo3);
        eprintln!("foo4: {:?}", foo4);

        let foo0 = deserialize::<Foo>(Format::Json, Version::V1_16_0, &json0);
    }

    fn serialize<T: Serialize>(format: Format, version: Version, value: &T) -> String {
        match format {
            Format::Json => {
                let mut buf = Vec::new();
                let mut serializer = serde_json::Serializer::new(&mut buf);
                let serializer = SubsonicSerializer::new(format, version, &mut serializer);
                value.serialize(serializer).unwrap();
                String::from_utf8(buf).unwrap()
            }
            Format::Xml => {
                let mut buffer = String::default();
                let serializer =
                    quick_xml::se::Serializer::with_root(&mut buffer, Some("subsonic-response"))
                        .unwrap();
                let serializer = SubsonicSerializer::new(format, version, serializer);
                value.serialize(serializer).unwrap();
                buffer
            }
        }
    }

    fn deserialize<T: for<'de> Deserialize<'de>>(
        format: Format,
        version: Version,
        content: &str,
    ) -> T {
        match format {
            Format::Json => {
                let mut de = serde_json::Deserializer::from_str(content);
                let de = SubsonicDeserializer::new(format, version, &mut de);
                T::deserialize(de).unwrap()
            }
            Format::Xml => {
                let mut de = quick_xml::de::Deserializer::from_str(content);
                let de = SubsonicDeserializer::new(format, version, &mut de);
                T::deserialize(de).unwrap()
            }
        }
    }
}
