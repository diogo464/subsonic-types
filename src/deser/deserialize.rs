use std::marker::PhantomData;

use serde::{
    de::{DeserializeSeed, Visitor},
    Deserialize,
};

use crate::common::{Format, Version};

pub trait SubsonicDeserialize<'de>: Sized {
    type Seed: DeserializeSeed<'de, Value = Self> + From<(Format, Version)>;
}
pub struct AnySeed<T>(PhantomData<T>);

impl<'de, T> DeserializeSeed<'de> for AnySeed<T>
where
    T: Deserialize<'de>,
{
    type Value = T;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer)
    }
}

impl<T> From<(Format, Version)> for AnySeed<T> {
    fn from(_: (Format, Version)) -> Self {
        Self(PhantomData)
    }
}

macro_rules! impl_subsonic_deserialize {
    ($($t:ty),*) => {
        $(
            impl<'de> SubsonicDeserialize<'de> for $t {
                type Seed = AnySeed<$t>;
            }
        )*
    };
}
impl_subsonic_deserialize!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, bool, String);

pub struct OptionSeed<T> {
    format: Format,
    version: Version,
    _marker: PhantomData<T>,
}

impl<T> From<(Format, Version)> for OptionSeed<T> {
    fn from((format, version): (Format, Version)) -> Self {
        Self {
            format,
            version,
            _marker: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for OptionSeed<T>
where
    T: SubsonicDeserialize<'de>,
{
    type Value = Option<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an option")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let seed = T::Seed::from((self.format, self.version));
        let value = seed.deserialize(deserializer)?;
        Ok(Some(value))
    }
}

impl<'de, T> DeserializeSeed<'de> for OptionSeed<T>
where
    T: SubsonicDeserialize<'de>,
{
    type Value = Option<T>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_option(self)
    }
}

impl<'de, T> SubsonicDeserialize<'de> for Option<T>
where
    T: SubsonicDeserialize<'de>,
{
    type Seed = OptionSeed<T>;
}

pub struct VecSeed<T> {
    format: Format,
    version: Version,
    _marker: PhantomData<T>,
}

impl<T> From<(Format, Version)> for VecSeed<T> {
    fn from((format, version): (Format, Version)) -> Self {
        Self {
            format,
            version,
            _marker: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for VecSeed<T>
where
    T: SubsonicDeserialize<'de>,
{
    type Value = Vec<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a sequence")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut vec = Vec::new();
        while let Some(value) = seq.next_element_seed(T::Seed::from((self.format, self.version)))? {
            vec.push(value);
        }
        Ok(vec)
    }
}

impl<'de, T> DeserializeSeed<'de> for VecSeed<T>
where
    T: SubsonicDeserialize<'de>,
{
    type Value = Vec<T>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(self)
    }
}

impl<'de, T> SubsonicDeserialize<'de> for Vec<T>
where
    T: SubsonicDeserialize<'de>,
{
    type Seed = VecSeed<T>;
}
