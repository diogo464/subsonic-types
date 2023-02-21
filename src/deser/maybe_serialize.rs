pub struct MaybeSerialize<T>(Option<T>);

impl<T> MaybeSerialize<T> {
    pub fn serialize(value: T) -> Self {
        Self(Some(value))
    }

    pub fn none() -> Self {
        Self(None)
    }

    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }
}

impl<T> Serialize for MaybeSerialize<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.0.as_ref() {
            Some(value) => value.serialize(serializer),
            None => unreachable!(),
        }
    }
}
