use super::{
    deserialize::{Error, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};

struct XmlString(String);

impl<'de> Deserialize<'de> for XmlString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StringVisitor).map(Self)
    }
}

struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string or a map with a single key \"$text\" or \"$value\"")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.to_owned())
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
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

impl<'de> Visitor<'de> for BoolVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a boolear or a string \"true\" or \"false\" or \"1\" or \"0\"")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
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
        A: MapAccess<'de>,
    {
        let v = StringVisitor::visit_map(StringVisitor, map)?;
        self.visit_str(&v)
    }
}

struct IntegerVisitor;

impl<'de> Visitor<'de> for IntegerVisitor {
    type Value = i64;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer or a string containing an integer")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        v.parse().map_err(E::custom)
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let v = StringVisitor::visit_map(StringVisitor, map)?;
        self.visit_str(&v)
    }
}

struct FloatVisitor;

impl<'de> Visitor<'de> for FloatVisitor {
    type Value = f64;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a float or a string containing a float")
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        v.parse().map_err(E::custom)
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let v = StringVisitor::visit_map(StringVisitor, map)?;
        self.visit_str(&v)
    }
}
