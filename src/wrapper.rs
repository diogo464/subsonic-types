use crate::{common::Version, Format, SubsonicDeserialize, SubsonicSerialize};

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

        impl<T> crate::SubsonicVersioned for $t<T>
        where
            T: crate::SubsonicVersioned,
        {
            const SINCE: Version = T::SINCE;
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

        impl<T> std::str::FromStr for $t<T>
        where
            T: std::str::FromStr,
        {
            type Err = <T as std::str::FromStr>::Err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                T::from_str(s).map(Self)
            }
        }
    };
}

impl_format_wrapper!(Json, Format::Json);
impl_format_wrapper!(Xml, Format::Xml);

impl<'de, T> serde::Deserialize<'de> for Json<T>
where
    T: SubsonicDeserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer, Format::Json).map(Self)
    }
}

struct XmlString(String);

impl<'de> serde::Deserialize<'de> for XmlString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(StringVisitor).map(Self)
    }
}

struct StringVisitor;

impl<'de> serde::de::Visitor<'de> for StringVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string or a map with a single key \"$text\" or \"$value\"")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v.to_owned())
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        use serde::de::Error;

        match map.next_entry::<XmlString, XmlString>()? {
            Some((XmlString(key), XmlString(value))) => {
                if key == "$text" || key == "$value" {
                    Ok(value)
                } else {
                    Err(Error::custom(
                        "expected a map with a single key \"$text\" or \"$value\"",
                    ))
                }
            }
            None => Err(Error::custom(
                "expected a map with a single key \"$text\" or \"$value\"",
            )),
        }
    }
}

struct BoolVisitor;

impl<'de> serde::de::Visitor<'de> for BoolVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a boolear or a string \"true\" or \"false\" or \"1\" or \"0\"")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "true" | "1" => Ok(true),
            "false" | "0" => Ok(false),
            _ => Err(E::custom(
                "expected \"true\" or \"false\" or \"1\" or \"0\"",
            )),
        }
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let v = StringVisitor::visit_map(StringVisitor, map)?;
        self.visit_str(&v)
    }
}

struct IntegerVisitor;

impl<'de> serde::de::Visitor<'de> for IntegerVisitor {
    type Value = i64;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer or a string containing an integer")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse().map_err(E::custom)
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let v = StringVisitor::visit_map(StringVisitor, map)?;
        self.visit_str(&v)
    }
}

struct FloatVisitor;

impl<'de> serde::de::Visitor<'de> for FloatVisitor {
    type Value = f64;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a float or a string containing a float")
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse().map_err(E::custom)
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let v = StringVisitor::visit_map(StringVisitor, map)?;
        self.visit_str(&v)
    }
}

struct SeqAccess<'de, A> {
    access: A,
    _phantom: std::marker::PhantomData<&'de ()>,
}

impl<'de, A> SeqAccess<'de, A> {
    fn new(access: A) -> Self {
        Self {
            access,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'de, A> serde::de::SeqAccess<'de> for SeqAccess<'de, A>
where
    A: serde::de::SeqAccess<'de>,
{
    type Error = A::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        self.access
            .next_element_seed(DeserializeSeedWrapper::new(seed))
    }
}

struct MapAccess<'de, A> {
    access: A,
    _phantom: std::marker::PhantomData<&'de ()>,
}

impl<'de, A> MapAccess<'de, A> {
    fn new(access: A) -> Self {
        Self {
            access,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'de, A> serde::de::MapAccess<'de> for MapAccess<'de, A>
where
    A: serde::de::MapAccess<'de>,
{
    type Error = A::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        // NOTE: keys should not need a seed wrapper since they should always be strings
        self.access.next_key_seed(seed)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        self.access
            .next_value_seed(DeserializeSeedWrapper::new(seed))
    }
}

struct DeserializeSeedWrapper<'de, T> {
    seed: T,
    _phantom: std::marker::PhantomData<&'de ()>,
}

impl<'de, T> DeserializeSeedWrapper<'de, T> {
    fn new(seed: T) -> Self {
        Self {
            seed,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'de, T> serde::de::DeserializeSeed<'de> for DeserializeSeedWrapper<'de, T>
where
    T: serde::de::DeserializeSeed<'de>,
{
    type Value = T::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.seed.deserialize(XmlDeserializer::new(deserializer))
    }
}

struct EnumAccess<'de, A> {
    access: A,
    _phantom: std::marker::PhantomData<&'de ()>,
}

impl<'de, A> EnumAccess<'de, A> {
    fn new(access: A) -> Self {
        Self {
            access,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'de, A> serde::de::EnumAccess<'de> for EnumAccess<'de, A>
where
    A: serde::de::EnumAccess<'de>,
{
    type Error = A::Error;
    type Variant = EnumVariantAccess<'de, A::Variant>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let (value, variant) = self.access.variant_seed(seed)?;
        Ok((value, EnumVariantAccess::new(variant)))
    }
}

struct EnumVariantAccess<'de, A> {
    access: A,
    _phantom: std::marker::PhantomData<&'de ()>,
}

impl<'de, A> EnumVariantAccess<'de, A> {
    fn new(access: A) -> Self {
        Self {
            access,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'de, A> serde::de::VariantAccess<'de> for EnumVariantAccess<'de, A>
where
    A: serde::de::VariantAccess<'de>,
{
    type Error = A::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        self.access.unit_variant()
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        self.access
            .newtype_variant_seed(DeserializeSeedWrapper::new(seed))
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.access.tuple_variant(len, VisitorWrapper::new(visitor))
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.access
            .struct_variant(fields, VisitorWrapper::new(visitor))
    }
}

struct VisitorWrapper<'de, V> {
    visitor: V,
    _phantom: std::marker::PhantomData<&'de ()>,
}

impl<'de, V> VisitorWrapper<'de, V> {
    fn new(visitor: V) -> Self {
        Self {
            visitor,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'de, V> serde::de::Visitor<'de> for VisitorWrapper<'de, V>
where
    V: serde::de::Visitor<'de>,
{
    type Value = V::Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an optional value")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visitor.visit_none()
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        self.visitor.visit_some(XmlDeserializer::new(deserializer))
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        self.visitor
            .visit_newtype_struct(XmlDeserializer::new(deserializer))
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        self.visitor.visit_seq(SeqAccess::new(seq))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        self.visitor.visit_map(MapAccess::new(map))
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        self.visitor.visit_enum(EnumAccess::new(data))
    }
}

struct XmlDeserializer<'de, D> {
    de: D,
    _marker: std::marker::PhantomData<&'de ()>,
}

impl<'de, D> XmlDeserializer<'de, D> {
    fn new(de: D) -> Self {
        Self {
            de,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'de, D> serde::Deserializer<'de> for XmlDeserializer<'de, D>
where
    D: serde::Deserializer<'de>,
{
    type Error = D::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.de.deserialize_any(visitor)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_bool(self.de.deserialize_any(BoolVisitor)?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i64(self.de.deserialize_any(IntegerVisitor)?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_f64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_f64(self.de.deserialize_any(FloatVisitor)?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.de.deserialize_char(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.de.deserialize_str(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.de.deserialize_string(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.de.deserialize_bytes(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.de.deserialize_byte_buf(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.de.deserialize_option(VisitorWrapper::new(visitor))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.de.deserialize_unit(visitor)
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.de
            .deserialize_unit_struct(name, VisitorWrapper::new(visitor))
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.de
            .deserialize_newtype_struct(name, VisitorWrapper::new(visitor))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.de.deserialize_seq(VisitorWrapper::new(visitor))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.de.deserialize_tuple(len, VisitorWrapper::new(visitor))
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.de
            .deserialize_tuple_struct(name, len, VisitorWrapper::new(visitor))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.de.deserialize_map(VisitorWrapper::new(visitor))
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.de
            .deserialize_struct(name, fields, VisitorWrapper::new(visitor))
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.de
            .deserialize_enum(name, variants, VisitorWrapper::new(visitor))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.de.deserialize_identifier(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.de.deserialize_ignored_any(visitor)
    }
}

impl<'de, T> serde::Deserialize<'de> for Xml<T>
where
    T: SubsonicDeserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(XmlDeserializer::new(deserializer), Format::Xml).map(Self)
    }
}
