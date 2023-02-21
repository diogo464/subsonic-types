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
