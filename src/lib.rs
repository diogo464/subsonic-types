#[macro_use]
pub(crate) mod traits;
pub(crate) mod helper;
pub(crate) mod wrapper;

pub(crate) use subsonic_macro::SubsonicType;
pub use traits::*;

pub mod common;
pub mod request;
pub mod response;

use response::Response;
pub use wrapper::{Json, Xml};

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
pub fn to_json(response: &Response) -> Result<String, SerdeError> {
    use serde::Serialize;
    #[derive(Serialize)]
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
///     let serialized = r#"{"subsonic-response":{"status":"ok","version":"1.16.1","license":{"valid":true}}}"#;
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

    #[derive(Deserialize)]
    struct SubsonicResponse {
        #[serde(rename = "subsonic-response")]
        subsonic_response: Json<Response>,
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
