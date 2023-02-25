use serde::{Serialize, Serializer};

use crate::common::{Format, Version};

pub trait SubsonicSerialize {
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

pub struct SubsonicSerializeWrapper<T>(pub T, pub Format, pub Version);
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

impl<T> SubsonicSerialize for Option<T>
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
        match self {
            Some(v) => v.serialize(serializer, format, version),
            None => serializer.serialize_none(),
        }
    }
}

impl<T> SubsonicSerialize for Vec<T>
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
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self {
            seq.serialize_element(&SubsonicSerializeWrapper(e, format, version))?;
        }
        seq.end()
    }
}
