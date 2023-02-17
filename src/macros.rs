macro_rules! impl_subsonic_for_serde {
    ($t:path) => {
        impl crate::SubsonicSerialize for $t {
            fn serialize<S>(
                &self,
                serializer: S,
                _: crate::Format,
            ) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                <Self as serde::Serialize>::serialize(self, serializer)
            }
        }

        impl<'de> crate::SubsonicDeserialize<'de> for $t {
            fn deserialize<D>(
                deserializer: D,
                _: crate::Format,
            ) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                <Self as serde::Deserialize<'de>>::deserialize(deserializer)
            }
        }
    };
}
