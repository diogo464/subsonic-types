use super::Format;

pub trait SubsonicSerialize {
    fn serialize<S>(&self, serializer: S, format: Format) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;
}

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

pub trait SubsonicDeserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D, format: Format) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>;
}

pub trait SubsonicType<'de>: SubsonicSerialize + SubsonicDeserialize<'de> {}

impl<'de, T: SubsonicSerialize + SubsonicDeserialize<'de>> SubsonicType<'de> for T {}

macro_rules! impl_subsonic_for_serde {
    ($t:path) => {
        impl $crate::deser::SubsonicSerialize for $t {
            fn serialize<S>(
                &self,
                serializer: S,
                _: $crate::deser::Format,
            ) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                <Self as serde::Serialize>::serialize(self, serializer)
            }
        }

        impl<'de> $crate::deser::SubsonicDeserialize<'de> for $t {
            fn deserialize<D>(
                deserializer: D,
                _: $crate::deser::Format,
            ) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                <Self as serde::Deserialize<'de>>::deserialize(deserializer)
            }
        }
    };
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
    fn serialize<S>(&self, serializer: S, format: crate::deser::Format) -> Result<S::Ok, S::Error>
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
    fn deserialize<D>(deserializer: D, format: crate::deser::Format) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match format {
            Format::Json => {
                let value =
                    <Option<crate::deser::Json<T>> as serde::Deserialize>::deserialize(deserializer)?;
                Ok(value.map(crate::deser::Json::into_inner))
            }
            Format::Xml => {
                let value =
                    <Option<crate::deser::Xml<T>> as serde::Deserialize>::deserialize(deserializer)?;
                Ok(value.map(crate::deser::Xml::into_inner))
            }
        }
    }
}

impl<T> SubsonicSerialize for Vec<T>
where
    T: SubsonicSerialize,
{
    fn serialize<S>(&self, serializer: S, format: crate::deser::Format) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        match format {
            Format::Json => {
                for value in self {
                    seq.serialize_element(&crate::deser::Json::new(value))?;
                }
            }
            Format::Xml => {
                for value in self {
                    seq.serialize_element(&crate::deser::Xml::new(value))?;
                }
            }
        }
        seq.end()
    }
}

impl<'de, T> SubsonicDeserialize<'de> for Vec<T>
where
    T: SubsonicDeserialize<'de>,
{
    fn deserialize<D>(deserializer: D, format: Format) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match format {
            Format::Json => {
                let value = <Vec<crate::deser::Json<T>> as serde::Deserialize>::deserialize(deserializer)?;
                Ok(value.into_iter().map(crate::deser::Json::into_inner).collect())
            }
            Format::Xml => {
                let value = <Vec<crate::deser::Xml<T>> as serde::Deserialize>::deserialize(deserializer)?;
                Ok(value.into_iter().map(crate::deser::Xml::into_inner).collect())
            }
        }
    }
}
