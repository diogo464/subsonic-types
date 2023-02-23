use crate::common::Version;

use super::Format;

pub struct ValidationError;

pub trait SubsonicSerialize<'s> {
    type Input: 's;

    type Output: serde::Serialize + 's;

    fn prepare(input: Self::Input, format: Format, version: Version) -> Self::Output;
}

pub trait SubsonicIntermidiate<'de>: Sized {
    fn deserialize<D>(format: Format, deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>;
}

pub trait SubsonicDeserialize<'de>: Sized {
    type Intermidiate: SubsonicIntermidiate<'de>;

    fn validate(
        intermidiate: Self::Intermidiate,
        version: Version,
    ) -> Result<Self, ValidationError>;
}

macro_rules! impl_subsonic_serialize_for_copy {
    ($($type:ty),*) => {
        $(
            impl<'s> $crate::deser::SubsonicSerialize<'s> for $type {
                type Input = &'s $type;
                type Output = $type;

                fn prepare(input: Self::Input, _: $crate::deser::Format, _: $crate::common::Version) -> Self::Output {
                    *input
                }
            }
        )*
    };
}

macro_rules! impl_subsonic_serialize {
    ($($type:ty),*) => {
        $(
            impl<'s> $crate::deser::SubsonicSerialize<'s> for $type {
                type Input = &'s $type;
                type Output = &'s $type;

                fn prepare(input: Self::Input, _: $crate::deser::Format, _: $crate::common::Version) -> Self::Output {
                    input
                }
            }
        )*
    };
}

macro_rules! impl_subsonic_deserialize {
    ($($type:ty),*) => {
        $(
            impl<'de> $crate::deser::SubsonicIntermidiate<'de> for $type {
                fn deserialize<D>(_: $crate::deser::Format, deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    <$type as serde::Deserialize<'de>>::deserialize(deserializer)
                }
            }

            impl<'de> $crate::deser::SubsonicDeserialize<'de> for $type {
                type Intermidiate = $type;

                fn validate(
                    intermidiate: Self::Intermidiate,
                    _: $crate::common::Version,
                ) -> Result<Self, $crate::deser::ValidationError> {
                    Ok(intermidiate)
                }
            }
        )*
    };
}

macro_rules! impl_subsonic {
    ($($type:ty),*) => {
        impl_subsonic_serialize!($($type),*);
        impl_subsonic_deserialize!($($type),*);
    };
}

impl_subsonic_serialize_for_copy!(bool, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
impl_subsonic_serialize!(String);
impl_subsonic_deserialize!(bool, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, String);

impl<'s, T> SubsonicSerialize<'s> for Option<T>
where
    T: SubsonicSerialize<'s, Input = &'s T> + 's,
{
    type Input = &'s Self;

    type Output = Option<T::Output>;

    fn prepare(input: Self::Input, format: Format, version: Version) -> Self::Output {
        input
            .as_ref()
            .map(|value| T::prepare(value, format, version))
    }
}

pub struct VecOutput<'s, T> {
    data: &'s [T],
    version: Version,
    format: Format,
}

impl<'s, T> serde::Serialize for VecOutput<'s, T>
where
    T: SubsonicSerialize<'s, Input = &'s T> + 's,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(self.data.len()))?;
        for item in self.data {
            seq.serialize_element(&T::prepare(item, self.format, self.version))?;
        }
        seq.end()
    }
}

impl<'s, T> SubsonicSerialize<'s> for Vec<T>
where
    T: SubsonicSerialize<'s, Input = &'s T> + 's,
{
    type Input = &'s Self;

    type Output = VecOutput<'s, T>;

    fn prepare(input: Self::Input, format: Format, version: Version) -> Self::Output {
        VecOutput {
            data: input,
            version,
            format,
        }
    }
}

impl<'de, T> SubsonicIntermidiate<'de> for Option<T>
where
    T: SubsonicDeserialize<'de>,
{
    type Output = Option<T>;

    fn deserialize<D>(format: Format, deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
    }

    fn validate(self, version: Version) -> Result<Self::Output, ValidationError> {
        todo!()
    }
}

impl<'de, T> SubsonicDeserialize<'de> for Option<T>
where
    T: SubsonicDeserialize<'de>,
{
    fn deserialize<D>(format: Format, version: Version, deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor<T> {
            format: Format,
            version: Version,
            _phantom: std::marker::PhantomData<T>,
        }

        impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
        where
            T: SubsonicDeserialize<'de>,
        {
            type Value = Option<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an optional value")
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Ok(Some(T::deserialize(
                    self.format,
                    self.version,
                    deserializer,
                )?))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(None)
            }
        }

        deserializer.deserialize_option(Visitor {
            format,
            version,
            _phantom: std::marker::PhantomData,
        })
    }
}

impl<'de, T> SubsonicDeserialize<'de> for Vec<T>
where
    T: SubsonicDeserialize<'de>,
{
    fn deserialize<D>(format: Format, version: Version, deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Seed<T> {
            format: Format,
            version: Version,
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T> Clone for Seed<T> {
            fn clone(&self) -> Self {
                Seed {
                    format: self.format,
                    version: self.version,
                    _phantom: std::marker::PhantomData,
                }
            }
        }

        impl<'de, T> serde::de::DeserializeSeed<'de> for Seed<T>
        where
            T: SubsonicDeserialize<'de>,
        {
            type Value = T;

            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                T::deserialize(self.format, self.version, deserializer)
            }
        }

        struct Visitor<T> {
            format: Format,
            version: Version,
            _phantom: std::marker::PhantomData<T>,
        }

        impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
        where
            T: SubsonicDeserialize<'de>,
        {
            type Value = Vec<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a vector")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let seed = Seed {
                    format: self.format,
                    version: self.version,
                    _phantom: std::marker::PhantomData,
                };
                let mut vec = Vec::new();
                while let Some(item) = seq.next_element_seed(seed.clone())? {
                    vec.push(item);
                }
                Ok(vec)
            }
        }

        deserializer.deserialize_seq(Visitor {
            format,
            version,
            _phantom: std::marker::PhantomData,
        })
    }
}
