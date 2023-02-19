use quick_xml::events::attributes::Attributes;
use subsonic_macro::SubsonicType;
use subsonic_types::{
    common::{DateTime, Version},
    response::{Child, Directory, License, Response, ResponseBody, ResponseStatus},
    Json, Xml,
};

pub use subsonic_types::{Format, SubsonicDeserialize, SubsonicSerialize};

struct Printer;

impl<'de> serde::Deserialize<'de> for Printer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PrinterVisitor;

        impl<'de> serde::de::Visitor<'de> for PrinterVisitor {
            type Value = Printer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("anything")
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                println!("i64: {}", v);
                Ok(Printer)
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                println!("u64: {}", v);
                Ok(Printer)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                println!("str: {}", v);
                Ok(Printer)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                println!("some");
                deserializer.deserialize_any(PrinterVisitor)
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                println!("none");
                Ok(Printer)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                println!("seq");
                while let Some(_) = seq.next_element::<Printer>()? {}
                println!("end seq");
                Ok(Printer)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                println!("map");
                while let Some(_) = map.next_entry::<Printer, Printer>()? {}
                println!("end map");
                Ok(Printer)
            }

            fn visit_enum<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::EnumAccess<'de>,
            {
                println!("enum");
                Ok(Printer)
            }
        }

        deserializer.deserialize_any(PrinterVisitor)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct Foo {
        x: u32,
        #[serde(flatten)]
        body: Option<Bar>,
    }

    #[derive(Serialize, Deserialize)]
    enum Bar {
        Int(u32),
        Float(f32),
    }

    let foo = Foo {
        x: 5,
        body: Some(Bar::Float(2.1)),
    };
    let serialized = serde_json::to_string(&foo)?;
    println!("{}", serialized);

    return Ok(());

    // use subsonic_types::{
    //     common::Version,
    //     response::{License, Response, ResponseBody},
    // };
    // let response = Response::ok(
    //     Version::V1_16_1,
    //     ResponseBody::License(License {
    //         valid: true,
    //         ..Default::default()
    //     }),
    // );
    // let serialized =
    //     r#"{"subsonic-response":{"status":"ok","version":"1.16.1","license":{"valid":true}}}"#;
    // let deserialized:License = subsonic_types::from_json(serialized)?;
    // assert_eq!(response, deserialized);
    // return Ok(());

    // let response = Response {
    //     status: ResponseStatus::Ok,
    //     version: Version::new(1, 16, 1),
    //     body: Some(ResponseBody::License(License {
    //         valid: true,
    //         email: Some("mail@example.com".into()),
    //         ..Default::default()
    //     })),
    // };

    // let expected = Response::ok(
    //     Version::new(1, 4, 0),
    //     ResponseBody::Directory(Directory {
    //         id: "11".into(),
    //         parent: Some("1".into()),
    //         name: "Arrival".into(),
    //         child: vec![
    //             Child {
    //                 id: "111".into(),
    //                 parent: Some("11".into()),
    //                 title: "Dancing Queen".into(),
    //                 is_dir: false,
    //                 album: Some("Arrival".into()),
    //                 artist: Some("ABBA".into()),
    //                 track: Some(7),
    //                 year: Some(1978),
    //                 genre: Some("Pop".into()),
    //                 cover_art: Some("24".into()),
    //                 size: Some(8421341),
    //                 content_type: Some("audio/mpeg".into()),
    //                 suffix: Some("mp3".into()),
    //                 duration: Some(146),
    //                 bit_rate: Some(128),
    //                 path: Some("ABBA/Arrival/Dancing Queen.mp3".into()),
    //                 ..Default::default()
    //             },
    //             Child {
    //                 id: "112".into(),
    //                 parent: Some("11".into()),
    //                 title: "Money, Money, Money".into(),
    //                 is_dir: false,
    //                 album: Some("Arrival".into()),
    //                 artist: Some("ABBA".into()),
    //                 track: Some(7),
    //                 year: Some(1978),
    //                 genre: Some("Pop".into()),
    //                 cover_art: Some("25".into()),
    //                 size: Some(4910028),
    //                 content_type: Some("audio/flac".into()),
    //                 suffix: Some("flac".into()),
    //                 transcoded_content_type: Some("audio/mpeg".into()),
    //                 transcoded_suffix: Some("mp3".into()),
    //                 duration: Some(208),
    //                 bit_rate: Some(128),
    //                 path: Some("ABBA/Arrival/Money, Money, Money.mp3".into()),
    //                 ..Default::default()
    //             },
    //         ],
    //         ..Default::default()
    //     }),
    // );
    //let output = subsonic_types::to_json(&expected)?;
    //println!("{}", output);

    let xml = r#"
    <subsonic-response status="ok" version="1.8.0">
        <artist id="5432" name="AC/DC" coverArt="ar-5432" albumCount="15">
            <album id="11047" name="Back In Black" coverArt="al-11047" songCount="10" created="2004-11-08T23:33:11" duration="2534" artist="AC/DC" artistId="5432"/>
            <album id="11048" name="Black Ice" coverArt="al-11048" songCount="15" created="2008-10-30T09:20:52" duration="3332" artist="AC/DC" artistId="5432"/>
            <album id="11049" name="Blow up your Video" coverArt="al-11049" songCount="10" created="2004-11-27T19:22:45" duration="2578" artist="AC/DC" artistId="5432"/>
            <album id="11050" name="Flick Of The Switch" coverArt="al-11050" songCount="10" created="2004-11-27T19:22:51" duration="2222" artist="AC/DC" artistId="5432"/>
            <album id="11051" name="Fly On The Wall" coverArt="al-11051" songCount="10" created="2004-11-27T19:22:57" duration="2405" artist="AC/DC" artistId="5432"/>
            <album id="11052" name="For Those About To Rock" coverArt="al-11052" songCount="10" created="2004-11-08T23:35:02" duration="2403" artist="AC/DC" artistId="5432"/>
            <album id="11053" name="High Voltage" coverArt="al-11053" songCount="8" created="2004-11-27T20:23:32" duration="2414" artist="AC/DC" artistId="5432"/>
            <album id="10489" name="Highway To Hell" coverArt="al-10489" songCount="12" created="2009-06-15T09:41:54" duration="2745" artist="AC/DC" artistId="5432"/>
            <album id="11054" name="If You Want Blood..." coverArt="al-11054" songCount="1" created="2004-11-27T20:23:32" duration="304" artist="AC/DC" artistId="5432"/>
        </artist>
    </subsonic-response>
    "#;

    let xml = r#"
    <subsonic-response status="ok" version="1.14.0">
        <albumInfo>
            <notes>
                Surrender is the third album from The Chemical Brothers and was released on June 22, 1999. It features Noel Gallagher (Oasis), Hope Sandoval (ex Mazzy Star) and Bernard Sumner (New Order) as guest vocalists. Leeds band The Sunshine Underground took their name from the sixth track on the album. It was certified 2x Platinum by the BPI on September 30, 2005. The song 'Asleep from Day' was used in a commercial for the French airline Air France. <a target='_blank' href="http://www.last.fm/music/The+Chemical+Brothers/Surrender">Read more on Last.fm</a>.
            </notes>
            <musicBrainzId>a84b9fea-aee9-4e1f-b5a2-a5a23c673688</musicBrainzId>
            <lastFmUrl>http://www.last.fm/music/The+Chemical+Brothers/Surrender</lastFmUrl>
            <smallImageUrl>http://img2-ak.lst.fm/i/u/64s/1428ec66344849829440668951259baa.png</smallImageUrl>
            <mediumImageUrl>http://img2-ak.lst.fm/i/u/174s/1428ec66344849829440668951259baa.png</mediumImageUrl>
            <largeImageUrl>http://img2-ak.lst.fm/i/u/1428ec66344849829440668951259baa.png</largeImageUrl>
        </albumInfo>
    </subsonic-response>
"#;

    quick_xml::de::from_str::<Printer>(&xml)?;
    return Ok(());

    type JsonValue = serde_json::Value;
    type JsonMap = serde_json::Map<String, serde_json::Value>;
    const SKIP_NAMES: &'static [&'static str] = &["a"];

    fn map_insert_kv_arr(map: &mut JsonMap, key: &str, value: JsonValue) {
        match map.remove(key) {
            Some(prev) => match prev {
                serde_json::Value::Array(mut arr) => {
                    arr.push(value);
                    map.insert(String::from(key), JsonValue::Array(arr));
                }
                other @ _ => {
                    let mut arr = serde_json::Value::Array(Vec::new());
                    arr.as_array_mut().unwrap().push(other);
                    arr.as_array_mut().unwrap().push(value);
                    map.insert(String::from(key), arr);
                }
            },
            None => {
                map.insert(String::from(key), value);
            }
        }
    }

    fn map_insert_attrs(map: &mut JsonMap, attrs: Attributes) {
        for attr in attrs {
            let attr = attr.unwrap();
            let key = attr.key.local_name();
            let key = std::str::from_utf8(key.as_ref()).unwrap();
            let value = std::str::from_utf8(&attr.value).unwrap();
            map_insert_kv_arr(map, key, JsonValue::String(String::from(value)));
        }
    }

    #[derive(Debug)]
    struct Entry {
        // The key where this entry should be placed in the parent map.
        key: String,
        // The map corresponding to the object being built.
        val: EntryValue,
    }

    impl Entry {
        fn root() -> Self {
            Self {
                key: Default::default(),
                val: EntryValue::Map(JsonMap::new()),
            }
        }

        fn new(name: impl Into<String>) -> Self {
            Self {
                key: name.into(),
                val: EntryValue::Empty,
            }
        }
    }

    #[derive(Debug)]
    enum EntryValue {
        Empty,
        String(String),
        Map(JsonMap),
    }

    impl EntryValue {
        pub fn insert_kv(&mut self, key: &str, value: JsonValue) {
            match self {
                EntryValue::Map(map) => map_insert_kv_arr(map, key, value),
                EntryValue::Empty => {
                    let mut map = JsonMap::new();
                    map.insert(String::from(key), value);
                    *self = EntryValue::Map(map);
                }
                _ => panic!("Cannot insert into kv non-map {:#?}", self),
            }
        }

        pub fn insert_attributes(&mut self, attrs: quick_xml::events::attributes::Attributes) {
            match self {
                EntryValue::Map(map) => map_insert_attrs(map, attrs),
                EntryValue::Empty => {
                    let mut map = JsonMap::new();
                    map_insert_attrs(&mut map, attrs);
                    if !map.is_empty() {
                        *self = EntryValue::Map(map);
                    }
                }
                _ => panic!("Cannot insert attributes into non-map {:#?}", self),
            }
        }

        pub fn insert_text(&mut self, text: &str) {
            match self {
                EntryValue::String(s) => {
                    s.push_str(text);
                }
                EntryValue::Empty => {
                    *self = EntryValue::String(String::from(text));
                }
                _ => panic!("Cannot insert text into non-string {:#?}", self),
            }
        }
    }

    let mut stack: Vec<Entry> = Vec::with_capacity(8);
    let mut reader = quick_xml::Reader::from_str(xml);

    // Ignore text at the beginning and end of the document
    reader.trim_text(true);

    // Push the root entry
    stack.push(Entry::root());

    while let Ok(event) = reader.read_event() {
        println!("{:?}", event);
        match event {
            quick_xml::events::Event::Start(ev) => {
                let local_name = ev.local_name();
                let key = std::str::from_utf8(local_name.as_ref()).unwrap();
                if SKIP_NAMES.contains(&key) {
                    continue;
                }

                let mut entry = Entry::new(key);
                entry.val.insert_attributes(ev.attributes());
                stack.push(entry);
            }
            quick_xml::events::Event::End(ev) => {
                let local_name = ev.local_name();
                let key = std::str::from_utf8(local_name.as_ref()).unwrap();
                if SKIP_NAMES.contains(&key) {
                    continue;
                }

                let entry = stack.pop().expect("stack should not be empty");
                let parent = stack.last_mut().expect("stack should not be empty");
                let entry_key = entry.key;
                debug_assert_eq!(key, entry_key);
                let object = match entry.val {
                    EntryValue::String(v) => JsonValue::String(v),
                    EntryValue::Map(v) => JsonValue::Object(v),
                    _ => continue,
                };
                parent.val.insert_kv(&entry_key, object);
            }
            quick_xml::events::Event::Empty(ev) => {
                let local_name = ev.local_name();
                let key = std::str::from_utf8(local_name.as_ref()).unwrap();
                if SKIP_NAMES.contains(&key) {
                    continue;
                }

                let entry = stack.last_mut().expect("stack should not be empty");
                let mut map = serde_json::Map::default();
                map_insert_attrs(&mut map, ev.attributes());
                entry.val.insert_kv(key, JsonValue::Object(map));
            }
            quick_xml::events::Event::Text(ev) => {
                let value = ev.unescape().unwrap();
                let entry = stack.last_mut().expect("stack should not be empty");
                entry.val.insert_text(&value);
            }
            quick_xml::events::Event::Eof => break,
            _ => {}
        }
    }

    assert_eq!(stack.len(), 1);
    let root = stack.pop().expect("stack should not be empty");
    let root = serde_json::Value::Object(match root.val {
        EntryValue::Map(m) => m,
        _ => panic!("Root should be a map"),
    });
    let root = serde_json::to_string_pretty(&root).unwrap();
    println!("{}", root);

    let deserialized = subsonic_types::from_json(&root)?;
    println!("{:#?}", deserialized);

    Ok(())
}
mod helper {
    #[allow(unused)]
    pub fn is_none<'a, T, U>(v: T) -> bool
    where
        T: AsRef<&'a Option<U>>,
        U: 'a,
    {
        v.as_ref().is_none()
    }
}

#[derive(Debug)]
pub struct SerdeError(Box<dyn std::error::Error>);

impl std::fmt::Display for SerdeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for SerdeError {}

impl From<serde_json::Error> for SerdeError {
    fn from(error: serde_json::Error) -> Self {
        Self(Box::new(error))
    }
}

impl From<quick_xml::DeError> for SerdeError {
    fn from(error: quick_xml::DeError) -> Self {
        Self(Box::new(error))
    }
}

pub fn to_json(response: &Response) -> Result<String, SerdeError> {
    use serde::Serialize;
    #[derive(Debug, Clone, PartialEq, Serialize)]
    struct SubsonicResponse<'a> {
        pub subsonic_response: Json<&'a Response>,
    }
    let wrapper = Json::new(response);
    let response = SubsonicResponse {
        subsonic_response: wrapper,
    };
    Ok(serde_json::to_string(&response)?)
}

pub fn to_xml(response: &Response) -> Result<String, SerdeError> {
    use serde::Serialize;
    let wrapper = Xml::new(response);
    let mut buffer = String::default();
    let serializer = quick_xml::se::Serializer::with_root(&mut buffer, Some("subsonic-response"))?;
    wrapper.serialize(serializer)?;
    Ok(buffer)
}

pub fn from_json(json: &str) -> Result<Response, SerdeError> {
    use serde::Deserialize;
    #[derive(Debug, Clone, PartialEq, Deserialize)]
    struct SubsonicResponse {
        pub subsonic_response: Json<Response>,
    }
    let response: SubsonicResponse = serde_json::from_str(json)?;
    Ok(response.subsonic_response.into_inner())
}

pub fn from_xml(xml: &str) -> Result<Response, SerdeError> {
    let response: Xml<Response> = quick_xml::de::from_str(xml)?;
    Ok(response.into_inner())
}

// pub fn to_json<S: SubsonicSerialize>(value: &S) -> Result<String, serde_json::Error> {
//     let wrapper = Json::new(value);
//     serde_json::to_string(&wrapper)
// }

// pub fn to_xml<S: SubsonicSerialize>(value: &S) -> Result<String, quick_xml::DeError> {
//     use serde::Serialize;
//     let mut buffer = String::default();
//     let wrapper = Xml::new(value);
//     let mut serializer = quick_xml::se::Serializer::new(&mut buffer);
//     serializer.indent(' ', 4);
//     wrapper.serialize(serializer)?;
//     Ok(buffer)
// }

pub fn to_xml_with_root<S: SubsonicSerialize>(
    value: &S,
    root: Option<&str>,
) -> Result<String, quick_xml::DeError> {
    use serde::Serialize;
    let mut buffer = String::default();
    let wrapper = Xml::new(value);
    let mut serializer = quick_xml::se::Serializer::with_root(&mut buffer, root)?;
    serializer.indent(' ', 4);
    wrapper.serialize(serializer)?;
    Ok(buffer)
}
