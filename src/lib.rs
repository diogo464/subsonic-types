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
    use std::collections::BTreeMap;

    use crate::common::{Format, Version};
    use crate::obj::{Constraint, FromValue, Object, ObjectDecoder, ObjectEncoder, ToValue, Value};
    use crate::response::Genre;

    #[derive(Debug, Default, PartialEq)]
    struct Foo {
        server: String,
        genre: Genre,
    }

    impl ToValue for Genre {
        fn to_value(&self, format: Format, version: Version) -> crate::obj::Result<Value> {
            let mut object = Object::default();
            let mut encoder = ObjectEncoder::new(&mut object, format, version);
            encoder.encode_attr_with(
                "songCount",
                &self.song_count,
                Constraint::default().with_version(Version::V1_10_2),
            )?;
            encoder.encode_attr_with(
                "albumCount",
                &self.album_count,
                Constraint::default().with_version(Version::V1_10_2),
            )?;
            encoder.encode_attr("name", &self.name)?;
            Ok(Value::Object(object))
        }
    }

    impl FromValue for Genre {
        fn from_value(value: Value, format: Format, version: Version) -> crate::obj::Result<Self> {
            let mut value = value.expect_object()?;
            let mut decoder = ObjectDecoder::new(&mut value, format, version);
            let song_count = decoder.decode_attr_with(
                "songCount",
                Constraint::default().with_version(Version::V1_10_2),
            )?;
            let album_count = decoder.decode_attr_with(
                "albumCount",
                Constraint::default().with_version(Version::V1_10_2),
            )?;
            let name = decoder.decode_attr("name")?;
            Ok(Self {
                song_count,
                album_count,
                name,
            })
        }
    }

    impl ToValue for Foo {
        fn to_value(&self, format: Format, version: Version) -> crate::obj::Result<Value> {
            let mut object = Object::default();
            let mut encoder = ObjectEncoder::new(&mut object, format, version);
            encoder.encode_attr("server", &self.server)?;
            encoder.encode_attr("genre", &self.genre)?;
            Ok(Value::Object(object))
        }
    }

    impl FromValue for Foo {
        fn from_value(value: Value, format: Format, version: Version) -> crate::obj::Result<Self> {
            let mut value = value.expect_object()?;
            let mut decoder = ObjectDecoder::new(&mut value, format, version);
            let server = decoder.decode_attr("server")?;
            let genre = decoder.decode_value("genre")?;
            Ok(Self { server, genre })
        }
    }

    #[derive(serde::Deserialize)]
    enum Test {
        Null,
        Nested(Box<Test>),
        Map(BTreeMap<String, Test>),
    }

    impl serde::Serialize for Test {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match self {
                Test::Null => serializer.serialize_none(),
                Test::Nested(nested) => nested.serialize(serializer),
                Test::Map(m) => m.serialize(serializer),
            }
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

        let test = Test::Nested(Box::new(Test::Null));
        eprintln!("test: {}", serde_json::to_string(&test).unwrap());

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

    fn serialize<T: ToValue>(format: Format, version: Version, value: &T) -> String {
        match format {
            Format::Json => {
                let mut buf = Vec::new();
                let mut serializer = serde_json::Serializer::new(&mut buf);
                let value = value.to_value(format, version).unwrap();
                serde::Serialize::serialize(&value, &mut serializer).unwrap();
                String::from_utf8(buf).unwrap()
            }
            Format::Xml => {
                let mut buffer = String::default();
                let serializer =
                    quick_xml::se::Serializer::with_root(&mut buffer, Some("subsonic-response"))
                        .unwrap();
                let value = &value.to_value(format, version).unwrap();
                serde::Serialize::serialize(value, serializer).unwrap();
                buffer
            }
        }
    }

    fn deserialize<T: FromValue>(format: Format, version: Version, content: &str) -> T {
        match format {
            Format::Json => {
                let mut de = serde_json::Deserializer::from_str(content);
                let value: Value = serde::Deserialize::deserialize(&mut de).unwrap();
                T::from_value(value, format, version).unwrap()
            }
            Format::Xml => {
                let mut de = quick_xml::de::Deserializer::from_str(content);
                let value: Value = serde::Deserialize::deserialize(&mut de).unwrap();
                T::from_value(value, format, version).unwrap()
            }
        }
    }
}
