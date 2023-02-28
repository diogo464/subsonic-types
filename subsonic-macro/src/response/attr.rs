use syn::Result;

pub use crate::attr::*;
use crate::version::Version;

pub struct ContainerAttr {
    /// This container implements Serialize/Deserialize and the implementation should
    /// delegate to those traits.
    pub serde: bool,
}

impl ContainerAttr {
    pub fn from_attrs(attrs: &[syn::Attribute]) -> Result<Self> {
        let metas = obtain_meta_list(attrs)?;
        let mut serde = false;

        for meta in metas {
            match meta {
                syn::Meta::Path(p) if SERDE == p => {
                    serde = true;
                }
                _ => {}
            }
        }

        Ok(Self { serde })
    }
}

pub struct FieldAttr {
    /// The name to use for the field in the serialized format.
    /// Translates to `#[serde(rename = "...")]`.
    pub rename: Option<String>,
    /// Is this an xml attribute?
    /// On the xml side, this translates to `#[serde(rename = "@...")]`.
    /// On the json side, this is ignored.
    pub attribute: bool,
    /// Should the field be flattened
    pub flatten: bool,
    /// Is this an xml value?
    /// If it is then this translates to `#[serde(rename="$value")]`.
    /// In json this translates to `#[serde(rename = "value")]`.
    /// This option is incompatible with the `flatten`, `choice`, `attribute` and `rename` options.
    pub value: bool,
    pub choice: bool,
    /// The version since this field was added.
    pub since: Option<Version>,
}

impl FieldAttr {
    pub fn from_attrs(attrs: &[syn::Attribute]) -> Result<Self> {
        let metas = obtain_meta_list(attrs)?;
        let mut rename = None;
        let mut attribute = false;
        let mut flatten = false;
        let mut value = false;
        let mut choice = false;
        let mut since = None;

        for meta in metas {
            match meta {
                syn::Meta::NameValue(nv) if RENAME == nv.path => {
                    if let syn::Lit::Str(s) = &nv.lit {
                        rename = Some(s.value());
                    }
                }
                syn::Meta::Path(p) if ATTRIBUTE == p => {
                    attribute = true;
                }
                syn::Meta::Path(p) if FLATTEN == p => {
                    flatten = true;
                }
                syn::Meta::Path(p) if VALUE == p => {
                    value = true;
                }
                syn::Meta::Path(p) if CHOICE == p => {
                    choice = true;
                }
                syn::Meta::NameValue(nv) if SINCE == nv.path => {
                    if let syn::Lit::Str(s) = &nv.lit {
                        since = Version::parse(&s.value())
                            .ok_or_else(|| syn::Error::new_spanned(nv, format!("Invalid version")))
                            .map(Some)?;
                    }
                }
                _ => return Err(syn::Error::new_spanned(meta, "Invalid subsonic attribute")),
            }
        }

        if choice && (flatten || attribute || value || rename.is_some()) {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "Choice fields cannot have flatten, attribute, value or rename attributes",
            ));
        }

        Ok(Self {
            rename,
            attribute,
            flatten,
            value,
            choice,
            since,
        })
    }
}
