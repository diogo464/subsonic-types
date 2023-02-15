use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use subsonic_types::SubsonicType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    Json,
    QuickXml,
}

pub trait SubsonicType<'de>: Sized {
    fn deserialize<D>(format: Format, deserialize: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>;

    fn serialize<S>(&self, format: Format, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Json<T>(T);

impl<T> serde::Serialize for Json<T>
where
    T: for<'a> SubsonicType<'a>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(Format::Json, serializer)
    }
}

impl<'de, T> serde::Deserialize<'de> for Json<T>
where
    T: SubsonicType<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(Format::Json, deserializer).map(Json)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuickXml<T>(T);

impl<T> serde::Serialize for QuickXml<T>
where
    T: for<'a> SubsonicType<'a>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(Format::QuickXml, serializer)
    }
}

impl<'de, T> serde::Deserialize<'de> for QuickXml<T>
where
    T: SubsonicType<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(Format::QuickXml, deserializer).map(QuickXml)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SubsonicType)]
pub struct Response2 {
    #[subsonic(xml(rename = "@status"), json(), common())]
    pub status: String,
    pub version: String,
    pub application_version: String,
}

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct Response {
//     pub status: String,
//     pub version: String,
//     pub body: ResponseBody,
// }

// impl<'de> SubsonicType<'de> for Response {
//     fn deserialize<D>(format: Format, deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         match format {
//             Format::Json => {
//                 #[derive(Deserialize)]
//                 struct Json {
//                     status: String,
//                     version: String,
//                     #[serde(flatten)]
//                     body: ResponseBody,
//                 }
//                 Json::deserialize(deserializer).map(|json| Response {
//                     status: json.status,
//                     version: json.version,
//                     body: json.body,
//                 })
//             }
//             Format::QuickXml => {
//                 #[derive(Deserialize)]
//                 struct Xml {
//                     #[serde(rename = "@status")]
//                     status: String,
//                     #[serde(rename = "@version")]
//                     version: String,
//                     #[serde(flatten)]
//                     body: ResponseBody,
//                 }
//                 Xml::deserialize(deserializer).map(|xml| Response {
//                     status: xml.status,
//                     version: xml.version,
//                     body: xml.body,
//                 })
//             }
//         }
//     }

//     fn serialize<S>(&self, format: Format, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         match format {
//             Format::Json => {
//                 #[derive(Serialize)]
//                 struct Json<'a> {
//                     status: &'a str,
//                     version: &'a str,
//                     #[serde(flatten)]
//                     body: &'a ResponseBody,
//                 }
//                 Json {
//                     status: &self.status,
//                     version: &self.version,
//                     body: &self.body,
//                 }
//                 .serialize(serializer)
//             }
//             Format::QuickXml => {
//                 #[derive(Serialize)]
//                 struct Xml<'a> {
//                     #[serde(rename = "@status")]
//                     status: &'a str,
//                     #[serde(rename = "@version")]
//                     version: &'a str,
//                     #[serde(flatten)]
//                     body: &'a ResponseBody,
//                 }
//                 Xml {
//                     status: &self.status,
//                     version: &self.version,
//                     body: &self.body,
//                 }
//                 .serialize(serializer)
//             }
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResponseBody {
    License(License),
}

const _: () = {
    enum ResponseBodyJson {}

    impl<'de> SubsonicType<'de> for ResponseBody {
        fn deserialize<D>(format: Format, deserialize: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            todo!()
        }

        fn serialize<S>(&self, format: Format, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            todo!()
        }
    }
};

impl From<License> for ResponseBody {
    fn from(license: License) -> Self {
        ResponseBody::License(license)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub struct License {
    pub valid: bool,
    pub email: Option<String>,
    pub licence_expires: Option<DateTime<Utc>>,
    pub trial_expires: Option<DateTime<Utc>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let response = Response {
    //     status: "ok".to_string(),
    //     version: "1.16.1".to_string(),
    //     body: ResponseBody::from(License {
    //         valid: true,
    //         ..Default::default()
    //     }),
    // };

    // // let output = serde_json::to_string(&response)?;
    // let mut output = String::new();
    // let mut serializer =
    //     quick_xml::se::Serializer::with_root(&mut output, Some("subsonic-response"))?;
    // serializer.indent(' ', 4);
    // QuickXml(response).serialize(serializer)?;

    // println!("{}", output);

    Ok(())
}
