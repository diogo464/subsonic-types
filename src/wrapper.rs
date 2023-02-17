use crate::{Format, SubsonicDeserialize, SubsonicSerialize};

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
