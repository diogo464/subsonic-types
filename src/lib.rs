mod types;
pub use types::*;

pub(crate) use subsonic_macro::SubsonicType;

pub trait SubsonicSerialize<'a>: 'a {
    type ToJson: serde::Serialize + From<&'a Self>;
    type ToXml: serde::Serialize + From<&'a Self>;
}

pub trait SubsonicDeserialize: Sized {
    type FromJson: for<'de> serde::Deserialize<'de> + Into<Self>;
    type FromXml: for<'de> serde::Deserialize<'de> + Into<Self>;
}

pub trait SubsonicType<'a>: SubsonicSerialize<'a> + SubsonicDeserialize {}

impl<'a, T: SubsonicSerialize<'a> + SubsonicDeserialize> SubsonicType<'a> for T {}

#[derive(Debug, Clone, PartialEq)]
pub struct Json<'r, T>(&'r T);

impl<'r, T> serde::Serialize for Json<'r, T>
where
    T: for<'a> SubsonicSerialize<'a>,
{
    fn serialize<'s, S>(&'s self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let wrapper = <T as SubsonicSerialize<'s>>::ToJson::from(&self.0);
        wrapper.serialize(serializer)
    }
}

macro_rules! impl_subsonic_serde {
    ($t:path) => {
        impl<'a> SubsonicSerialize<'a> for $t {
            type ToJson = &'a $t;
            type ToXml = &'a $t;
        }

        impl SubsonicDeserialize for $t {
            type FromJson = $t;
            type FromXml = $t;
        }
    };

    (@ $t:ident) => {};
}

const _: () = {
    #[derive(serde::Serialize)]
    struct ToJson<'a, T>(&'a Option<T>);
    impl<'a, T: SubsonicSerialize<'a>> From<&'a Option<T>> for ToJson<'a, T::ToJson> {
        fn from(value: &'a Option<T>) -> Self {
            ToJson(
                value
                    .as_ref()
                    .map(|v| <T as SubsonicSerialize<'a>>::ToJson::from(v)),
            )
        }
    }

    #[derive(serde::Serialize)]
    struct ToXml<'a, T>(&'a Option<T>);
    impl<'a, T: SubsonicSerialize<'a>> From<&'a Option<T>> for ToXml<'a, T::ToXml> {
        fn from(value: &'a Option<T>) -> Self {
            ToXml(
                value
                    .as_ref()
                    .map(|v| <T as SubsonicSerialize<'a>>::ToXml::from(v)),
            )
        }
    }

    impl<'a, T> SubsonicSerialize<'a> for Option<T>
    where
        T: SubsonicSerialize<'a>,
    {
        type ToJson = ToJson<'a, T::ToJson>;
        type ToXml = ToXml<'a, T::ToXml>;
    }

    #[derive(serde::Deserialize)]
    struct FromJson<T>(Option<T>);
    impl<T: SubsonicDeserialize> From<FromJson<T::FromJson>> for Option<T> {
        fn from(value: FromJson<T::FromJson>) -> Self {
            value.0.map(Into::into)
        }
    }

    #[derive(serde::Deserialize)]
    struct FromXml<T>(Option<T>);
    impl<T: SubsonicDeserialize> From<FromXml<T::FromXml>> for Option<T> {
        fn from(value: FromXml<T::FromXml>) -> Self {
            value.0.map(Into::into)
        }
    }

    impl<T> SubsonicDeserialize for Option<T>
    where
        T: SubsonicDeserialize,
    {
        type FromJson = FromJson<T::FromJson>;
        type FromXml = FromXml<T::FromXml>;
    }
};

const _: () = {
    #[derive(serde::Serialize)]
    struct ToJson<'a, T>(&'a Vec<T>);
    impl<'a, T: SubsonicSerialize<'a>> From<&'a Vec<T>> for ToJson<'a, T::ToJson> {
        fn from(value: &'a Vec<T>) -> Self {
            ToJson(
                value
                    .iter()
                    .map(|v| <T as SubsonicSerialize<'a>>::ToJson::from(v))
                    .collect(),
            )
        }
    }

    #[derive(serde::Serialize)]
    struct ToXml<'a, T>(&'a Vec<T>);
    impl<'a, T: SubsonicSerialize<'a>> From<&'a Vec<T>> for ToXml<'a, T::ToXml> {
        fn from(value: &'a Vec<T>) -> Self {
            ToXml(
                value
                    .iter()
                    .map(|v| <T as SubsonicSerialize<'a>>::ToXml::from(v))
                    .collect(),
            )
        }
    }

    impl<'a, T> SubsonicSerialize<'a> for Vec<T>
    where
        T: SubsonicSerialize<'a>,
    {
        type ToJson = ToJson<'a, T::ToJson>;
        type ToXml = ToXml<'a, T::ToXml>;
    }

    #[derive(serde::Deserialize)]
    struct FromJson<T>(Vec<T>);
    impl<T: SubsonicDeserialize> From<FromJson<T::FromJson>> for Vec<T> {
        fn from(value: FromJson<T::FromJson>) -> Self {
            value.0.into_iter().map(Into::into).collect()
        }
    }

    #[derive(serde::Deserialize)]
    struct FromXml<T>(Vec<T>);
    impl<T: SubsonicDeserialize> From<FromXml<T::FromXml>> for Vec<T> {
        fn from(value: FromXml<T::FromXml>) -> Self {
            value.0.into_iter().map(Into::into).collect()
        }
    }

    impl<T> SubsonicDeserialize for Vec<T>
    where
        T: SubsonicDeserialize,
    {
        type FromJson = FromJson<T::FromJson>;
        type FromXml = FromXml<T::FromXml>;
    }
};

impl_subsonic_serde!(u8);
impl_subsonic_serde!(u16);
impl_subsonic_serde!(u32);
impl_subsonic_serde!(u64);
impl_subsonic_serde!(i8);
impl_subsonic_serde!(i16);
impl_subsonic_serde!(i32);
impl_subsonic_serde!(i64);
impl_subsonic_serde!(f32);
impl_subsonic_serde!(f64);
impl_subsonic_serde!(bool);
impl_subsonic_serde!(String);
impl_subsonic_serde!(chrono::DateTime<chrono::Utc>);
impl_subsonic_serde!(types::ResponseStatus);
impl_subsonic_serde!(types::Version);
impl_subsonic_serde!(types::UserRating);
impl_subsonic_serde!(types::AverageRating);
impl_subsonic_serde!(types::MediaType);
impl_subsonic_serde!(types::PodcastStatus);
impl_subsonic_serde!(types::ResponseBody);
