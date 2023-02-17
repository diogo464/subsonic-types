pub enum Format {
    Json,
    Xml,
}

pub trait SubsonicSerialize {
    fn serialize<S>(&self, serializer: S, format: Format) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;
}

pub trait SubsonicDeserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D, format: Format) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>;
}

pub trait SubsonicType<'de>: SubsonicSerialize + SubsonicDeserialize<'de> {}

impl<'de, T: SubsonicSerialize + SubsonicDeserialize<'de>> SubsonicType<'de> for T {}

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

macro_rules! impl_format_wrapper {
    ($t:ident, $f:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $t<T>(T);

        impl<T> $t<T> {
            pub fn new(value: T) -> Self {
                Self(value)
            }

            pub fn into_inner(self) -> T {
                self.0
            }

            pub fn as_inner(&self) -> &T {
                &self.0
            }

            pub fn as_inner_mut(&mut self) -> &mut T {
                &mut self.0
            }

            pub fn map<U, F>(self, f: F) -> $t<U>
            where
                F: FnOnce(T) -> U,
            {
                $t(f(self.0))
            }
        }

        impl<T> AsRef<T> for $t<T> {
            fn as_ref(&self) -> &T {
                &self.0
            }
        }

        impl<T> From<T> for $t<T> {
            fn from(value: T) -> Self {
                Self(value)
            }
        }

        impl<T> serde::Serialize for $t<T>
        where
            T: SubsonicSerialize,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                T::serialize(&self.0, serializer, $f)
            }
        }

        impl<'de, T> serde::Deserialize<'de> for $t<T>
        where
            T: SubsonicDeserialize<'de>,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                T::deserialize(deserializer, $f).map(Self)
            }
        }
    };
}

impl_format_wrapper!(Json, Format::Json);
impl_format_wrapper!(Xml, Format::Xml);

macro_rules! impl_subsonic_for_serde {
    ($t:path) => {
        impl SubsonicSerialize for $t {
            fn serialize<S>(&self, serializer: S, _: Format) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                <Self as serde::Serialize>::serialize(self, serializer)
            }
        }

        impl<'de> SubsonicDeserialize<'de> for $t {
            fn deserialize<D>(deserializer: D, _: Format) -> Result<Self, D::Error>
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
    fn serialize<S>(&self, serializer: S, format: crate::Format) -> Result<S::Ok, S::Error>
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
    fn deserialize<D>(deserializer: D, format: crate::Format) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match format {
            Format::Json => {
                let value =
                    <Option<crate::Json<T>> as serde::Deserialize>::deserialize(deserializer)?;
                Ok(value.map(crate::Json::into_inner))
            }
            Format::Xml => {
                let value =
                    <Option<crate::Xml<T>> as serde::Deserialize>::deserialize(deserializer)?;
                Ok(value.map(crate::Xml::into_inner))
            }
        }
    }
}

impl<T> SubsonicSerialize for Vec<T>
where
    T: SubsonicSerialize,
{
    fn serialize<S>(&self, serializer: S, format: crate::Format) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match format {
            Format::Json => {
                let value: &Vec<crate::Json<T>> =
                    unsafe { std::mem::transmute::<&Vec<T>, &Vec<crate::Json<T>>>(self) };
                <Vec<crate::Json<T>> as serde::Serialize>::serialize(value, serializer)
            }
            Format::Xml => {
                let value: &Vec<crate::Xml<T>> =
                    unsafe { std::mem::transmute::<&Vec<T>, &Vec<crate::Xml<T>>>(self) };
                <Vec<crate::Xml<T>> as serde::Serialize>::serialize(value, serializer)
            }
        }
    }
}

impl<'de, T> SubsonicDeserialize<'de> for Vec<T>
where
    T: SubsonicDeserialize<'de>,
{
    fn deserialize<D>(deserializer: D, format: crate::Format) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match format {
            Format::Json => {
                let value = <Vec<crate::Json<T>> as serde::Deserialize>::deserialize(deserializer)?;
                Ok(value.into_iter().map(crate::Json::into_inner).collect())
            }
            Format::Xml => {
                let value = <Vec<crate::Xml<T>> as serde::Deserialize>::deserialize(deserializer)?;
                Ok(value.into_iter().map(crate::Xml::into_inner).collect())
            }
        }
    }
}

fn macro_helper_is_none<'a, T, U>(v: T) -> bool
where
    T: AsRef<&'a Option<U>>,
    U: 'a,
{
    v.as_ref().is_none()
}
