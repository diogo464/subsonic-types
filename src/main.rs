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

impl<'a, T: serde::Serialize + 'a> SubsonicSerialize<'a> for T {
    type ToJson = &'a T;
    type ToXml = &'a T;
}

impl<T> SubsonicDeserialize for T
where
    T: for<'de> serde::Deserialize<'de>,
{
    type FromJson = T;
    type FromXml = T;
}

#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Foo {
    a: u32,
    b: u32,
}

#[derive(Debug, Default, Clone, PartialEq /*, SubsonicType */)]
struct License {
    pub valid: bool,
    pub email: Option<String>,
    pub license_expires: Option<String>,
    //#[subsonic(rename = "", attribute, optional)]
    pub trial_expires: Option<String>,
}

const _: () = {
    #[derive(serde::Serialize)]
    struct ToJson<'a> {
        pub valid: <bool as SubsonicSerialize<'a>>::ToJson,
        pub email: <Option<String> as SubsonicSerialize<'a>>::ToJson,
        pub license_expires: <Option<String> as SubsonicSerialize<'a>>::ToJson,
        pub trial_expires: <Option<String> as SubsonicSerialize<'a>>::ToJson,
        #[serde(skip_serializing)]
        pub __subsonic_phantom: std::marker::PhantomData<&'a ()>,
    }

    impl<'a> From<&'a License> for ToJson<'a> {
        fn from(value: &'a License) -> Self {
            Self {
                valid: From::from(&value.valid),
                email: From::from(&value.email),
                license_expires: From::from(&value.license_expires),
                trial_expires: From::from(&value.trial_expires),
                __subsonic_phantom: std::marker::PhantomData,
            }
        }
    }

    #[derive(serde::Deserialize)]
    struct FromJson {
        pub valid: <bool as SubsonicDeserialize>::FromJson,
        pub email: <Option<String> as SubsonicDeserialize>::FromJson,
        pub license_expires: <Option<String> as SubsonicDeserialize>::FromJson,
        pub trial_expires: <Option<String> as SubsonicDeserialize>::FromJson,
    }

    impl Into<License> for FromJson {
        fn into(self) -> License {
            License {
                valid: From::from(self.valid),
                email: From::from(self.email),
                license_expires: From::from(self.license_expires),
                trial_expires: From::from(self.trial_expires),
            }
        }
    }

    impl<'a> SubsonicSerialize<'a> for License {
        type ToJson = ToJson<'a>;
        type ToXml = ToJson<'a>;
    }

    impl SubsonicDeserialize for License {
        type FromJson = FromJson;
        type FromXml = FromJson;
    }
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let foo = Foo { a: 1, b: 2 };
    let json = to_json(&foo)?;
    println!("{}", json);
    let xml = to_xml(&foo)?;
    println!("{}", xml);

    let license = License::default();
    let json = to_json(&license)?;
    println!("{}", json);
    let xml = to_xml(&license)?;
    println!("{}", xml);

    Ok(())
}

pub fn to_json<'a, S: SubsonicSerialize<'a>>(value: &'a S) -> Result<String, serde_json::Error> {
    let wrapper = S::ToJson::from(value);
    serde_json::to_string(&wrapper)
}

pub fn to_xml<'a, S: SubsonicSerialize<'a>>(value: &'a S) -> Result<String, quick_xml::DeError> {
    let wrapper = S::ToXml::from(value);
    quick_xml::se::to_string(&wrapper)
}

// TODO:
// + Remake proc macro derive
// + Add Date type that wraps chrono::DateTime<Utc>
// + Strongly typed durations. Second and Millisecond types.
