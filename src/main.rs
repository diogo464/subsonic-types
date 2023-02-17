use subsonic_types::{Json, SubsonicSerialize, Xml};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn to_json<S: SubsonicSerialize>(value: &S) -> Result<String, serde_json::Error> {
    let wrapper = Json::new(value);
    serde_json::to_string(&wrapper)
}

pub fn to_xml<S: SubsonicSerialize>(value: &S) -> Result<String, quick_xml::DeError> {
    use serde::Serialize;
    let mut buffer = String::default();
    let wrapper = Xml::new(value);
    let mut serializer = quick_xml::se::Serializer::new(&mut buffer);
    serializer.indent(' ', 4);
    wrapper.serialize(serializer)?;
    Ok(buffer)
}

pub fn to_xml_with_root<S: SubsonicSerialize>(
    value: &S,
    root: &str,
) -> Result<String, quick_xml::DeError> {
    use serde::Serialize;
    let mut buffer = String::default();
    let wrapper = Xml::new(value);
    let mut serializer = quick_xml::se::Serializer::with_root(&mut buffer, Some(root))?;
    serializer.indent(' ', 4);
    wrapper.serialize(serializer)?;
    Ok(buffer)
}

// TODO:
// + Remake proc macro derive
// + Add Date type that wraps chrono::DateTime<Utc>
// + Strongly typed durations. Second and Millisecond types.
