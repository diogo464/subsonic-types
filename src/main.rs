pub enum Format {
    Json,
    Xml,
}

pub trait SubsonicSerialize {
    fn serialize<S>(&self, serializer: S, format: Format) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;
}

pub trait SubsonicDeserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D, format: Format) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>;
}

pub trait SubsonicType<'de>: SubsonicSerialize + SubsonicDeserialize<'de> {}

impl<'de, T: SubsonicSerialize + SubsonicDeserialize<'de>> SubsonicType<'de> for T {}

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

        impl<'de, T> serde::Deserialize<'de> for $t<T>
        where
            T: SubsonicDeserialize<'de>,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                T::deserialize(deserializer, $f).map(Self)
            }
        }
    };
}

impl_format_wrapper!(Json, Format::Json);
impl_format_wrapper!(Xml, Format::Xml);

pub fn macro_helper_is_none<'a, T, U>(v: T) -> bool
where
    T: AsRef<&'a Option<U>>,
    U: 'a,
{
    v.as_ref().is_none()
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

macro_rules! impl_subsonic_for_serde {
    ($t:path) => {
        impl SubsonicSerialize for $t {
            fn serialize<S>(&self, serializer: S, _: Format) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                <Self as serde::Serialize>::serialize(self, serializer)
            }
        }

        impl<'de> SubsonicDeserialize<'de> for $t {
            fn deserialize<D>(deserializer: D, _: Format) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                <Self as serde::Deserialize<'de>>::deserialize(deserializer)
            }
        }
    };
}
impl_subsonic_for_serde!(u8);
impl_subsonic_for_serde!(u16);
impl_subsonic_for_serde!(u32);
impl_subsonic_for_serde!(u64);
impl_subsonic_for_serde!(u128);
impl_subsonic_for_serde!(usize);
impl_subsonic_for_serde!(i8);
impl_subsonic_for_serde!(i16);
impl_subsonic_for_serde!(i32);
impl_subsonic_for_serde!(i64);
impl_subsonic_for_serde!(i128);
impl_subsonic_for_serde!(isize);
impl_subsonic_for_serde!(f32);
impl_subsonic_for_serde!(f64);
impl_subsonic_for_serde!(bool);
impl_subsonic_for_serde!(char);
impl_subsonic_for_serde!(String);

impl<T> SubsonicSerialize for Option<T>
where
    T: SubsonicSerialize,
{
    fn serialize<S>(&self, serializer: S, format: crate::Format) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Some(value) => value.serialize(serializer, format),
            None => serializer.serialize_none(),
        }
    }
}

impl<'de, T> SubsonicDeserialize<'de> for Option<T>
where
    T: SubsonicDeserialize<'de>,
{
    fn deserialize<D>(deserializer: D, format: crate::Format) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match format {
            Format::Json => {
                let value =
                    <Option<crate::Json<T>> as serde::Deserialize>::deserialize(deserializer)?;
                Ok(value.map(crate::Json::into_inner))
            }
            Format::Xml => {
                let value =
                    <Option<crate::Xml<T>> as serde::Deserialize>::deserialize(deserializer)?;
                Ok(value.map(crate::Xml::into_inner))
            }
        }
    }
}

impl<T> SubsonicSerialize for Vec<T>
where
    T: SubsonicSerialize,
{
    fn serialize<S>(&self, serializer: S, format: crate::Format) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match format {
            Format::Json => {
                let value: &Vec<crate::Json<T>> =
                    unsafe { std::mem::transmute::<&Vec<T>, &Vec<crate::Json<T>>>(self) };
                <Vec<crate::Json<T>> as serde::Serialize>::serialize(value, serializer)
            }
            Format::Xml => {
                let value: &Vec<crate::Xml<T>> =
                    unsafe { std::mem::transmute::<&Vec<T>, &Vec<crate::Xml<T>>>(self) };
                <Vec<crate::Xml<T>> as serde::Serialize>::serialize(value, serializer)
            }
        }
    }
}

impl<'de, T> SubsonicDeserialize<'de> for Vec<T>
where
    T: SubsonicDeserialize<'de>,
{
    fn deserialize<D>(deserializer: D, format: crate::Format) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match format {
            Format::Json => {
                let value = <Vec<crate::Json<T>> as serde::Deserialize>::deserialize(deserializer)?;
                Ok(value.into_iter().map(crate::Json::into_inner).collect())
            }
            Format::Xml => {
                let value = <Vec<crate::Xml<T>> as serde::Deserialize>::deserialize(deserializer)?;
                Ok(value.into_iter().map(crate::Xml::into_inner).collect())
            }
        }
    }
}

const _: () = {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(serde::Serialize)]
    struct ToJson<'a> {
        pub code: crate::Json<&'a u32>,
        pub message: crate::Json<&'a String>,
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
        pub code: crate::Json<u32>,
        pub message: crate::Json<String>,
    }

    impl Into<Status> for FromJson {
        fn into(self) -> Status {
            Status {
                code: crate::Json::into_inner(self.code),
                message: crate::Json::into_inner(self.message),
            }
        }
    }

    impl SubsonicSerialize for Status {
        fn serialize<S>(&self, serializer: S, format: Format) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match format {
                Format::Json => {
                    let value = ToJson::from(self);
                    value.serialize(serializer)
                }
                Format::Xml => {
                    let value = ToJson::from(self);
                    value.serialize(serializer)
                }
            }
        }
    }

    impl<'de> SubsonicDeserialize<'de> for Status {
        fn deserialize<D>(deserializer: D, format: Format) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            match format {
                Format::Json => {
                    let value = FromJson::deserialize(deserializer)?;
                    Ok(value.into())
                }
                Format::Xml => {
                    let value = FromJson::deserialize(deserializer)?;
                    Ok(value.into())
                }
            }
        }
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
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(serde::Serialize)]
    struct ToJson<'a> {
        pub valid: crate::Json<&'a bool>,
        pub email: crate::Json<&'a Option<String>>,
        pub license_expires: crate::Json<&'a Option<String>>,
        pub trial_expires: crate::Json<&'a Option<String>>,
        pub status: crate::Json<&'a Status>,
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
        pub valid: crate::Json<bool>,
        pub email: crate::Json<Option<String>>,
        pub license_expires: crate::Json<Option<String>>,
        pub trial_expires: crate::Json<Option<String>>,
        pub status: crate::Json<Status>,
    }

    impl Into<License> for FromJson {
        fn into(self) -> License {
            License {
                valid: crate::Json::into_inner(self.valid),
                email: crate::Json::into_inner(self.email),
                license_expires: crate::Json::into_inner(self.license_expires),
                trial_expires: crate::Json::into_inner(self.trial_expires),
                status: crate::Json::into_inner(self.status),
            }
        }
    }

    impl SubsonicSerialize for License {
        fn serialize<S>(&self, serializer: S, format: Format) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match format {
                Format::Json => {
                    let value = ToJson::from(self);
                    value.serialize(serializer)
                }
                Format::Xml => {
                    let value = ToJson::from(self);
                    value.serialize(serializer)
                }
            }
        }
    }

    impl<'de> SubsonicDeserialize<'de> for License {
        fn deserialize<D>(deserializer: D, format: Format) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            match format {
                Format::Json => {
                    let value = FromJson::deserialize(deserializer)?;
                    Ok(value.into())
                }
                Format::Xml => {
                    let value = FromJson::deserialize(deserializer)?;
                    Ok(value.into())
                }
            }
        }
    }
};

#[derive(subsonic_macro::SubsonicType)]
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
    #[serde(rename = "ua")]
    #[serde(skip)]
    a: u32,
}

#[derive(subsonic_macro::SubsonicType)]
enum Baaar {
    License(License),
    License2(License2),
}

enum Baar {
    License(License),
    License2(License2),
}
const _: () = {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(serde::Serialize)]
    enum ToJson<'a> {
        License(crate::Json<&'a License>),
        License2(crate::Json<&'a License2>),
        LePhantom(std::marker::PhantomData<&'a ()>),
    }

    impl<'a> From<&'a Baar> for ToJson<'a> {
        fn from(value: &'a Baar) -> Self {
            match value {
                Baar::License(value) => Self::License(crate::Json::from(value)),
                Baar::License2(value) => Self::License2(crate::Json::from(value)),
            }
        }
    }

    #[derive(serde::Deserialize)]
    enum FromJson {
        License(crate::Json<License>),
        License2(crate::Json<License2>),
    }

    impl Into<Baar> for FromJson {
        fn into(self) -> Baar {
            match self {
                FromJson::License(value) => Baar::License(crate::Json::into_inner(value)),
                FromJson::License2(value) => Baar::License2(crate::Json::into_inner(value)),
            }
        }
    }

    impl SubsonicSerialize for Baar {
        fn serialize<S>(&self, serializer: S, format: Format) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match format {
                Format::Json => {
                    let value = ToJson::from(self);
                    value.serialize(serializer)
                }
                Format::Xml => {
                    let value = ToJson::from(self);
                    value.serialize(serializer)
                }
            }
        }
    }

    impl<'de> SubsonicDeserialize<'de> for Baar {
        fn deserialize<D>(deserializer: D, format: Format) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            match format {
                Format::Json => {
                    let value = FromJson::deserialize(deserializer)?;
                    Ok(value.into())
                }
                Format::Xml => {
                    let value = FromJson::deserialize(deserializer)?;
                    Ok(value.into())
                }
            }
        }
    }
};

#[derive(subsonic_macro::SubsonicType)]
struct Resp {
    status: u32,
    #[subsonic(flatten)]
    body: Baaar,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let foo = Foo { a: 1, b: 2 };
    // let json = to_json(&foo)?;
    // println!("{}", json);
    // let xml = to_xml(&foo)?;
    // println!("{}", xml);

    let license2 = License2 {
        valid: true,
        email: None,
        license_expires: None,
        trial_expires: Some("uirutuerhjks".to_string()),
        status: Status {
            code: 200,
            message: Default::default(),
        },
    };
    let json = to_json(&license2)?;
    println!("{}", json);
    let xml = to_xml(&license2)?;
    println!("{}", xml);

    let baar = Baaar::License2(license2);
    let json = to_json(&baar)?;
    println!("{}", json);
    let xml = to_xml(&baar)?;
    println!("{}", xml);

    let resp = Resp {
        status: 200,
        body: baar,
    };
    let json = to_json(&resp)?;
    println!("{}", json);
    let xml = to_xml_with_root(&resp, "subsonic-response")?;
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

pub fn to_json<S: SubsonicSerialize>(value: &S) -> Result<String, serde_json::Error> {
    let wrapper = Json(value);
    serde_json::to_string(&wrapper)
}

pub fn to_xml<S: SubsonicSerialize>(value: &S) -> Result<String, quick_xml::DeError> {
    use serde::Serialize;
    let mut buffer = String::default();
    let wrapper = Xml(value);
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
    let wrapper = Xml(value);
    let mut serializer = quick_xml::se::Serializer::with_root(&mut buffer, Some(root))?;
    serializer.indent(' ', 4);
    wrapper.serialize(serializer)?;
    Ok(buffer)
}

// TODO:
// + Remake proc macro derive
// + Add Date type that wraps chrono::DateTime<Utc>
// + Strongly typed durations. Second and Millisecond types.
