use std::{borrow::Cow, collections::BTreeMap};

use crate::common::{Format, Version};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Error(Cow<'static, str>);

impl Error {
    pub fn new<E>(err: E) -> Self
    where
        E: std::error::Error,
    {
        Self(Cow::Owned(err.to_string()))
    }

    pub fn custom(msg: impl ToString) -> Self {
        Self(Cow::Owned(msg.to_string()))
    }

    pub fn unexpected_value(expected: &str, got: Value) -> Self {
        Self::custom(format!("expected {}, got {:?}", expected, got))
    }

    pub fn invalid_value(key: &str, msg: impl ToString) -> Self {
        Self::custom(format!("invalid value for {}: {}", key, msg.to_string()))
    }

    fn custom_static(msg: &'static str) -> Self {
        Self(Cow::Borrowed(msg))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for Error {}

pub trait ToValue {
    fn to_value(&self, format: Format, version: Version) -> Result<Value>;
}

pub trait FromValue: Default + Sized {
    fn from_value(value: Value, format: Format, version: Version) -> Result<Self>;
}

#[derive(Debug, Default, Clone)]
pub struct Constraint {
    version: Option<Version>,
    format: Option<Format>,
}

impl Constraint {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_version(mut self, version: Version) -> Self {
        self.version = Some(version);
        self
    }

    pub fn with_format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }

    fn allow(&self, format: Format, version: Version) -> bool {
        if let Some(f) = self.format {
            if f != format {
                return false;
            }
        }

        if let Some(v) = self.version {
            if v > version {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Attribute(String),
    String(String),
    Integer(i64),
    Boolean(bool),
    Float(f64),
    Array(Vec<Value>),
    Object(Object),
}

impl Value {
    pub fn expect_null(self) -> Result<()> {
        match self {
            Value::Null => Ok(()),
            _ => Err(Error::unexpected_value("null", self)),
        }
    }

    pub fn expect_attribute(self) -> Result<String> {
        match self {
            Value::Attribute(s) => Ok(s),
            _ => Err(Error::unexpected_value("attribute", self)),
        }
    }

    pub fn expect_string(self) -> Result<String> {
        match self {
            Value::String(s) => Ok(s),
            _ => Err(Error::unexpected_value("string", self)),
        }
    }

    pub fn expect_integer(self) -> Result<i64> {
        match self {
            Value::Integer(i) => Ok(i),
            _ => Err(Error::unexpected_value("integer", self)),
        }
    }

    pub fn expect_boolean(self) -> Result<bool> {
        match self {
            Value::Boolean(b) => Ok(b),
            _ => Err(Error::unexpected_value("boolean", self)),
        }
    }

    pub fn expect_float(self) -> Result<f64> {
        match self {
            Value::Float(f) => Ok(f),
            _ => Err(Error::unexpected_value("float", self)),
        }
    }

    pub fn expect_array(self) -> Result<Vec<Value>> {
        match self {
            Value::Array(a) => Ok(a),
            _ => Err(Error::unexpected_value("array", self)),
        }
    }

    pub fn expect_object(self) -> Result<Object> {
        match self {
            Value::Object(o) => Ok(o),
            _ => Err(Error::unexpected_value("object", self)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Object(BTreeMap<String, Value>);

impl Object {
    pub fn merge(&mut self, other: Object) {
        self.0.extend(other.0);
    }
}

#[derive(Debug)]
pub struct ObjectEncoder<'a> {
    object: &'a mut Object,
    format: Format,
    version: Version,
}

impl<'a> ObjectEncoder<'a> {
    pub fn new(object: &'a mut Object, format: Format, version: Version) -> Self {
        Self {
            object,
            format,
            version,
        }
    }

    pub fn encode_attr<T>(&mut self, key: &str, value: T) -> Result<()>
    where
        T: ToValue,
    {
        self.encode_attr_with(key, value, Default::default())
    }

    pub fn encode_attr_with<T>(&mut self, key: &str, value: T, constraint: Constraint) -> Result<()>
    where
        T: ToValue,
    {
        if !constraint.allow(self.format, self.version) {
            return Ok(());
        }
        let value = match value.to_value(self.format, self.version)? {
            Value::Null => return Ok(()),
            Value::Attribute(v) => v,
            Value::String(v) => v,
            Value::Integer(v) => v.to_string(),
            Value::Boolean(v) => v.to_string(),
            Value::Float(v) => v.to_string(),
            Value::Array(_) => {
                return Err(Error::custom_static("Cannot encode array as attribute"))
            }
            Value::Object(_) => {
                return Err(Error::custom_static("Cannot encode object as attribute"))
            }
        };
        let key = match self.format {
            Format::Json => key.to_string(),
            Format::Xml => format!("@{}", key),
        };
        self.object.0.insert(key, Value::Attribute(value));
        Ok(())
    }

    pub fn encode_value<T>(&mut self, key: &str, value: T) -> Result<()>
    where
        T: ToValue,
    {
        self.encode_value_with(key, value, Default::default())
    }

    pub fn encode_value_with<T>(
        &mut self,
        key: &str,
        value: T,
        constraint: Constraint,
    ) -> Result<()>
    where
        T: ToValue,
    {
        if !constraint.allow(self.format, self.version) {
            return Ok(());
        }
        let value = value.to_value(self.format, self.version)?;
        if !std::matches!(value, Value::Null) {
            self.object.0.insert(key.to_string(), value);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct ObjectDecoder<'a> {
    object: &'a mut Object,
    format: Format,
    version: Version,
}

impl<'a> ObjectDecoder<'a> {
    pub fn new(object: &'a mut Object, format: Format, version: Version) -> Self {
        Self {
            object,
            format,
            version,
        }
    }

    pub fn decode_attr<T>(&mut self, key: &str) -> Result<T>
    where
        T: FromValue,
    {
        self.decode_attr_with(key, Default::default())
    }

    pub fn decode_attr_with<T>(&mut self, key: &str, constraint: Constraint) -> Result<T>
    where
        T: FromValue,
    {
        if !constraint.allow(self.format, self.version) {
            return Ok(Default::default());
        }
        let key = match self.format {
            Format::Json => key.to_string(),
            Format::Xml => format!("@{}", key),
        };
        let value = match self.object.0.remove(&key) {
            Some(value) => value,
            None => Value::Null,
        };
        T::from_value(value, self.format, self.version).map_err(|e| Error::invalid_value(&key, e))
    }

    pub fn decode_value<T>(&mut self, key: &str) -> Result<T>
    where
        T: FromValue,
    {
        self.decode_value_with(key, Default::default())
    }

    pub fn decode_value_with<T>(&mut self, key: &str, constraint: Constraint) -> Result<T>
    where
        T: FromValue,
    {
        if !constraint.allow(self.format, self.version) {
            return Ok(Default::default());
        }
        let value = match self.object.0.remove(key) {
            Some(value) => value,
            None => Value::Null,
        };
        T::from_value(value, self.format, self.version).map_err(|e| Error::invalid_value(&key, e))
    }
}

impl serde::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Null => serializer.serialize_none(),
            Value::Attribute(v) => serializer.serialize_str(v.as_str()),
            Value::String(v) => serializer.serialize_str(v.as_str()),
            Value::Integer(v) => serializer.serialize_i64(*v),
            Value::Boolean(v) => serializer.serialize_bool(*v),
            Value::Float(v) => serializer.serialize_f64(*v),
            Value::Array(v) => v.serialize(serializer),
            Value::Object(v) => v.0.serialize(serializer),
        }
    }
}

// impl serde::Serialize for Object {
//     fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         use serde::ser::SerializeMap;
//         let mut map = serializer.serialize_map(Some(self.0.len()))?;
//         for (key, value) in self.0.iter() {
//             map.serialize_entry(key, value)?;
//         }
//         map.end()
//     }
// }

impl<'de> serde::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct V;
        impl<'de> serde::de::Visitor<'de> for V {
            type Value = Value;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a value")
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::String(v.to_string()))
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Integer(v))
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Integer(v as i64))
            }

            fn visit_bool<E>(self, v: bool) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Boolean(v))
            }

            fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Float(v))
            }

            fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Null)
            }

            fn visit_some<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                serde::Deserialize::deserialize(deserializer)
            }

            fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element()? {
                    values.push(value);
                }
                Ok(Value::Array(values))
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut values = BTreeMap::default();
                while let Some((key, value)) = map.next_entry()? {
                    values.insert(key, value);
                }
                Ok(Value::Object(Object(values)))
            }
        }
        deserializer.deserialize_any(V)
    }
}

impl<T> ToValue for &T
where
    T: ToValue,
{
    fn to_value(&self, format: Format, version: Version) -> Result<Value> {
        (*self).to_value(format, version)
    }
}

macro_rules! impl_to_value_integer {
    ($($t:ty),*) => {
        $(
            impl ToValue for $t {
                fn to_value(&self, _: Format, _: Version) -> Result<Value> {
                    Ok(Value::Integer(*self as i64))
                }
            }
        )*
    };
}
impl_to_value_integer!(i8, i16, i32, i64, u8, u16, u32, u64);

macro_rules! impl_from_value_for_integer {
    ($($t:ty),*) => {
        $(
            impl FromValue for $t {
                fn from_value(value: Value, format: Format, version: Version) -> Result<Self> {
                    match value {
                        Value::Attribute(attr) => attr.parse().map_err(Error::custom),
                        Value::Integer(v) => Ok(v as $t),
                        Value::Object(mut v) if format == Format::Xml => {
                            let mut decoder = ObjectDecoder::new(&mut v, format, version);
                            decoder.decode_value("$value")
                        }
                        v => Err(Error::unexpected_value(std::stringify!($t), v)),
                    }
                }
            }
        )*
    };
}
impl_from_value_for_integer!(i8, i16, i32, i64, u8, u16, u32, u64);

impl ToValue for str {
    fn to_value(&self, _: Format, _: Version) -> Result<Value> {
        Ok(Value::String(self.to_string()))
    }
}

impl ToValue for String {
    fn to_value(&self, _: Format, _: Version) -> Result<Value> {
        Ok(Value::String(self.clone()))
    }
}

impl FromValue for String {
    fn from_value(value: Value, format: Format, version: Version) -> Result<Self> {
        match value {
            Value::Attribute(attr) => Ok(attr.to_string()),
            Value::String(v) => Ok(v),
            Value::Object(mut v) if format == Format::Xml => {
                let mut decoder = ObjectDecoder::new(&mut v, format, version);
                decoder.decode_value("$value")
            }
            v => Err(Error::unexpected_value("String", v)),
        }
    }
}

impl ToValue for bool {
    fn to_value(&self, _: Format, _: Version) -> Result<Value> {
        Ok(Value::Boolean(*self))
    }
}

impl FromValue for bool {
    fn from_value(value: Value, format: Format, version: Version) -> Result<Self> {
        match value {
            Value::Attribute(attr) => match attr.as_str() {
                "true" | "1" => Ok(true),
                "false" | "0" => Ok(false),
                _ => Err(Error::custom(format!("Invalid boolean value: {}", attr))),
            },
            Value::Boolean(v) => Ok(v),
            Value::Object(mut v) if format == Format::Xml => {
                let mut decoder = ObjectDecoder::new(&mut v, format, version);
                decoder.decode_value("$value")
            }
            v => Err(Error::unexpected_value("bool", v)),
        }
    }
}

impl<T> ToValue for Option<T>
where
    T: ToValue,
{
    fn to_value(&self, format: Format, version: Version) -> Result<Value> {
        match self {
            Some(v) => v.to_value(format, version),
            None => Ok(Value::Null),
        }
    }
}

impl<T> FromValue for Option<T>
where
    T: FromValue,
{
    fn from_value(value: Value, format: Format, version: Version) -> Result<Self> {
        match value {
            Value::Null => Ok(None),
            v => T::from_value(v, format, version).map(Some),
        }
    }
}

impl<T> ToValue for Vec<T>
where
    T: ToValue,
{
    fn to_value(&self, format: Format, version: Version) -> Result<Value> {
        let mut values = Vec::new();
        for v in self {
            values.push(v.to_value(format, version)?);
        }
        Ok(Value::Array(values))
    }
}

impl<T> FromValue for Vec<T>
where
    T: FromValue,
{
    fn from_value(value: Value, format: Format, version: Version) -> Result<Self> {
        match value {
            Value::Array(values) => {
                let mut result = Vec::new();
                for v in values {
                    result.push(T::from_value(v, format, version)?);
                }
                Ok(result)
            }
            v => Err(Error::unexpected_value("Vec", v)),
        }
    }
}
