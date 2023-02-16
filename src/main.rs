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

#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Foo {
    a: u32,
    b: u32,
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Status {
    code: u32,
    message: String,
}

const _: () = {
    #[derive(serde::Serialize)]
    struct ToJson<'a> {
        pub code: <u32 as SubsonicSerialize<'a>>::ToJson,
        pub message: <String as SubsonicSerialize<'a>>::ToJson,
        #[serde(skip_serializing)]
        pub __subsonic_phantom: std::marker::PhantomData<&'a ()>,
    }

    impl<'a> From<&'a Status> for ToJson<'a> {
        fn from(value: &'a Status) -> Self {
            Self {
                code: From::from(&value.code),
                message: From::from(&value.message),
                __subsonic_phantom: std::marker::PhantomData,
            }
        }
    }

    #[derive(serde::Deserialize)]
    struct FromJson {
        pub code: <u32 as SubsonicDeserialize>::FromJson,
        pub message: <String as SubsonicDeserialize>::FromJson,
    }

    impl Into<Status> for FromJson {
        fn into(self) -> Status {
            Status {
                code: From::from(self.code),
                message: From::from(self.message),
            }
        }
    }

    impl<'a> SubsonicSerialize<'a> for Status {
        type ToJson = ToJson<'a>;
        type ToXml = ToJson<'a>;
    }

    impl SubsonicDeserialize for Status {
        type FromJson = FromJson;
        type FromXml = FromJson;
    }
};

#[derive(Debug, Default, Clone, PartialEq /*, SubsonicType */)]
struct License {
    pub valid: bool,
    pub email: Option<String>,
    pub license_expires: Option<String>,
    //#[subsonic(rename = "", attribute, optional, flatten)]
    pub trial_expires: Option<String>,
    pub status: Status,
}

/// Proc macro steps
/// ```ignore
/// enum Format {
///     Json,
///     Xml,
/// }
///
/// enum Mode {
///     Serialize,
///     Deserialize,
/// }
///
/// struct Container<'a> {
///     input: &'a syn::DeriveInput,
///     format: Format,
///     mode: Mode,
///     patched: syn::DeriveInput,
/// }
///
/// impl<'a> Container<'a> {
///     fn from_input(input: &'a syn::DeriveInput,format: Format, mode: Mode) -> Result<Self> {
///         let mut patched = input.clone();
///     }
/// }
///
/// ````

const _: () = {
    #[derive(serde::Serialize)]
    struct ToJson<'a> {
        pub valid: <bool as SubsonicSerialize<'a>>::ToJson,
        pub email: <Option<String> as SubsonicSerialize<'a>>::ToJson,
        pub license_expires: <Option<String> as SubsonicSerialize<'a>>::ToJson,
        pub trial_expires: <Option<String> as SubsonicSerialize<'a>>::ToJson,
        pub status: <Status as SubsonicSerialize<'a>>::ToJson,
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
                status: From::from(&value.status),
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
        pub status: <Status as SubsonicDeserialize>::FromJson,
    }

    impl Into<License> for FromJson {
        fn into(self) -> License {
            License {
                valid: Into::into(self.valid),
                email: Into::into(self.email),
                license_expires: Into::into(self.license_expires),
                trial_expires: Into::into(self.trial_expires),
                status: Into::into(self.status),
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

#[derive(subsonic_macro::SubsonicType2)]
struct License2 {
    pub valid: bool,
    pub email: Option<String>,
    pub license_expires: Option<String>,
    #[subsonic(rename = "yaaaaa", attribute, optional)]
    pub trial_expires: Option<String>,
    pub status: Status,
}

#[derive(serde::Serialize)]
struct Fooo {
    #[serde(rename="ua")]
    #[serde(skip)]

    a: u32
}

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

    let json = serde_json::to_string(&Json(&license))?;
    println!("{}", json);

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
