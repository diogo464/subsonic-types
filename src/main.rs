#![feature(negative_impls)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use subsonic_types::SubsonicType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    Json,
    QuickXml,
}

pub trait SubsonicSerialize {
    fn serialize<S>(&self, format: Format, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;
}

pub trait SubsonicDeserialize<'de>: Sized {
    fn deserialize<D>(format: Format, deserialize: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>;
}

pub trait SubsonicType<'de>: SubsonicSerialize + SubsonicDeserialize<'de> {}

impl<'de, T: SubsonicSerialize + SubsonicDeserialize<'de>> SubsonicType<'de> for T {}

macro_rules! wrapper_impl {
    ($t:ident, $f:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $t<T>(T);

        impl<T> $t<T> {
            pub fn into_inner(self) -> T {
                self.0
            }
        }

        impl<T> serde::Serialize for $t<T>
        where
            T: $crate::SubsonicSerialize,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                <Self as $crate::SubsonicSerialize>::serialize(self, $f, serializer)
            }
        }
    };
}

wrapper_impl!(Json, Format::Json);
wrapper_impl!(QuickXml, Format::QuickXml);

// macro_rules! impl_subsonic_serialize {
//     ($t:ty) => {
//         impl SubsonicSerialize for $t {
//             fn serialize<S>(&self, format: Format, serializer: S) -> Result<S::Ok, S::Error>
//             where
//                 S: serde::Serializer,
//             {
//                 <Self as serde::Serialize>::serialize(self, serializer)
//             }
//         }
//     };
//     (@ $t:ty) => {
//         impl<T: serde::Serialize> SubsonicSerialize for $t {
//             fn serialize<S>(&self, format: Format, serializer: S) -> Result<S::Ok, S::Error>
//             where
//                 S: serde::Serializer,
//             {
//                 <Self as serde::Serialize>::serialize(self, serializer)
//             }
//         }
//     };
// }

// impl_subsonic_serialize!(bool);
// impl_subsonic_serialize!(String);
// impl_subsonic_serialize!(@ Option<T>);
// impl_subsonic_serialize!(DateTime<Utc>);

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, SubsonicType)]
#[subsonic(common(rename_all = "camelCase"))]
pub struct License {
    pub valid: bool,
    pub email: Option<String>,
    pub licence_expires: Option<DateTime<Utc>>,
    pub trial_expires: Option<DateTime<Utc>>,
}

// #[derive(Debug, Default, Clone, PartialEq, Eq, Hash, SubsonicType)]
// pub struct Response {
//     status: String,
//     license: License,
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let license = License {
        valid: true,
        ..Default::default()
    };
    let output = serde_json::to_string(&Json(&license))?;
    println!("{output}");

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
