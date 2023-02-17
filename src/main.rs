use subsonic_macro::SubsonicType;
use subsonic_types::{
    common::{DateTime, Version},
    response::{License, Response, ResponseBody, ResponseStatus},
    Json, Xml,
};

pub use subsonic_types::{Format, SubsonicDeserialize, SubsonicSerialize};

#[derive(Debug, SubsonicType)]
struct Foo {
    #[subsonic(rename = "@field_a")]
    field_a: bool,
    field_b: bool,
}

#[derive(Debug, Default, Clone, PartialEq, SubsonicType)]
pub struct License2 {
    #[subsonic(attribute)]
    pub valid: bool,
    #[subsonic(attribute, optional)]
    pub email: Option<String>,
    #[subsonic(attribute, optional)]
    pub licence_expires: Option<DateTime>,
    #[subsonic(attribute, optional)]
    pub trial_expires: Option<DateTime>,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub struct Response2 {
    #[subsonic(attribute)]
    pub status: ResponseStatus,
    #[subsonic(attribute)]
    pub version: Version,
    #[subsonic(flatten, complex)]
    pub body: ResponseBody2,
}

#[derive(Debug, Clone, PartialEq, SubsonicType)]
pub enum ResponseBody2 {
    License(License2),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let license2 = License2 {
    //     valid: true,
    //     ..Default::default()
    // };
    // let xml = to_xml_with_root(&license2, Some("license"))?;
    // println!("XML: {}", xml);
    // let value = quick_xml::de::from_str::<Xml<License2>>(&xml)?;
    // println!("XML: {:?}", value);

    // let response2 = Response2 {
    //     status: ResponseStatus::Ok,
    //     version: Version::V1_16_1,
    //     body: ResponseBody2::License(license2),
    // };
    // let xml = to_xml_with_root(&response2, Some("subsonic-response"))?;
    // println!("XML: {}", xml);
    // let value = quick_xml::de::from_str::<Xml<Response2>>(&xml)?;
    // println!("XML: {:?}", value);

    let response = Response::ok(
        Version::V1_16_1,
        ResponseBody::License(License {
            valid: true,
            ..Default::default()
        }),
    );
    println!("{}", to_json(&response)?);

    println!("XML: {}", to_xml(&response)?);

    let xml = to_xml_with_root(&response, Some("subsonic-response"))?;
    println!("XML: {}", xml);
    let value = quick_xml::de::from_str::<Xml<Response>>(&xml)?;
    println!("XML: {:?}", value);

    assert_eq!(response, from_json(&to_json(&response)?)?);
    assert_eq!(response, from_xml(&to_xml(&response)?)?);

    let xml = to_xml(&response)?;
    let mut reader = quick_xml::Reader::from_str(&xml);
    while let Ok(ev) = reader.read_event() {
        match ev {
            quick_xml::events::Event::Eof => break,
            _ => {}
        }
        println!("{:?}", ev);
    }

    Ok(())
}

#[allow(unused)]
fn macro_helper_is_none<'a, T, U>(v: T) -> bool
where
    T: AsRef<&'a Option<U>>,
    U: 'a,
{
    v.as_ref().is_none()
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

pub fn to_json(response: &Response) -> Result<String, SerdeError> {
    use serde::Serialize;
    #[derive(Debug, Clone, PartialEq, Serialize)]
    struct SubsonicResponse<'a> {
        pub subsonic_response: Json<&'a Response>,
    }
    let wrapper = Json::new(response);
    let response = SubsonicResponse {
        subsonic_response: wrapper,
    };
    Ok(serde_json::to_string(&response)?)
}

pub fn to_xml(response: &Response) -> Result<String, SerdeError> {
    use serde::Serialize;
    let wrapper = Xml::new(response);
    let mut buffer = String::default();
    let serializer = quick_xml::se::Serializer::with_root(&mut buffer, Some("subsonic-response"))?;
    wrapper.serialize(serializer)?;
    Ok(buffer)
}

pub fn from_json(json: &str) -> Result<Response, SerdeError> {
    use serde::Deserialize;
    #[derive(Debug, Clone, PartialEq, Deserialize)]
    struct SubsonicResponse {
        pub subsonic_response: Json<Response>,
    }
    let response: SubsonicResponse = serde_json::from_str(json)?;
    Ok(response.subsonic_response.into_inner())
}

pub fn from_xml(xml: &str) -> Result<Response, SerdeError> {
    let response: Xml<Response> = quick_xml::de::from_str(xml)?;
    Ok(response.into_inner())
}

// pub fn to_json<S: SubsonicSerialize>(value: &S) -> Result<String, serde_json::Error> {
//     let wrapper = Json::new(value);
//     serde_json::to_string(&wrapper)
// }

// pub fn to_xml<S: SubsonicSerialize>(value: &S) -> Result<String, quick_xml::DeError> {
//     use serde::Serialize;
//     let mut buffer = String::default();
//     let wrapper = Xml::new(value);
//     let mut serializer = quick_xml::se::Serializer::new(&mut buffer);
//     serializer.indent(' ', 4);
//     wrapper.serialize(serializer)?;
//     Ok(buffer)
// }

pub fn to_xml_with_root<S: SubsonicSerialize>(
    value: &S,
    root: Option<&str>,
) -> Result<String, quick_xml::DeError> {
    use serde::Serialize;
    let mut buffer = String::default();
    let wrapper = Xml::new(value);
    let mut serializer = quick_xml::se::Serializer::with_root(&mut buffer, root)?;
    serializer.indent(' ', 4);
    wrapper.serialize(serializer)?;
    Ok(buffer)
}
