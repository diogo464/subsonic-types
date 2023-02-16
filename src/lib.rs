mod types;
pub use types::*;

pub(crate) use subsonic_macro::SubsonicType;

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

impl<T: SubsonicSerialize> SubsonicSerialize for &T {
    fn serialize<S>(&self, format: Format, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        <T as SubsonicSerialize>::serialize(self, format, serializer)
    }
}

impl<T: SubsonicSerialize> SubsonicSerialize for Vec<T> {
    fn serialize<S>(&self, format: Format, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match format {
            Format::Json => {
                let vec: &Vec<Json<T>> = unsafe { std::mem::transmute(self) };
                <Vec<Json<T>> as serde::Serialize>::serialize(vec, serializer)
            }
            Format::QuickXml => {
                let vec: &Vec<QuickXml<T>> = unsafe { std::mem::transmute(self) };
                <Vec<QuickXml<T>> as serde::Serialize>::serialize(vec, serializer)
            }
        }
    }
}

impl<'de, T: SubsonicDeserialize<'de>> SubsonicDeserialize<'de> for Vec<T> {
    fn deserialize<D>(format: Format, deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match format {
            Format::Json => {
                let vec: Vec<Json<T>> = serde::Deserialize::deserialize(deserializer)?;
                Ok(vec.into_iter().map(Json::into_inner).collect())
            }
            Format::QuickXml => {
                let vec: Vec<QuickXml<T>> = serde::Deserialize::deserialize(deserializer)?;
                Ok(vec.into_iter().map(QuickXml::into_inner).collect())
            }
        }
    }
}

impl<'de, T: SubsonicSerialize + SubsonicDeserialize<'de>> SubsonicType<'de> for T {}

macro_rules! wrapper_impl {
    ($t:ident, $f:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $t<T>(pub T);

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
                <T as $crate::SubsonicSerialize>::serialize(&self.0, $f, serializer)
            }
        }

        impl<'de, T> serde::Deserialize<'de> for $t<T>
        where
            T: $crate::SubsonicDeserialize<'de>,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Ok(Self(<T as $crate::SubsonicDeserialize<'de>>::deserialize(
                    $f,
                    deserializer,
                )?))
            }
        }
    };
}

wrapper_impl!(Json, Format::Json);
wrapper_impl!(QuickXml, Format::QuickXml);

pub fn to_json<T>(value: T) -> Result<String, serde_json::Error>
where
    T: SubsonicSerialize,
{
    serde_json::to_string(&Json(value))
}

pub fn from_json<'de, T>(s: &'de str) -> Result<T, serde_json::Error>
where
    T: SubsonicDeserialize<'de>,
{
    let Json(value) = serde_json::from_str(s)?;
    Ok(value)
}

pub fn to_xml<T>(value: T) -> Result<String, quick_xml::DeError>
where
    T: SubsonicSerialize,
{
    use serde::Serialize;
    let mut buffer = String::new();
    let mut serializer =
        quick_xml::se::Serializer::with_root(&mut buffer, Some("subsonic-response"))?;
    serializer.indent(' ', 4);
    QuickXml(value).serialize(serializer)?;
    Ok(buffer)
}
