use syn::Result;

pub use crate::attr::*;
use crate::{util, version::Version};

use super::format::Format;

pub struct FieldAttr {
    /// The name to use for the field in the serialized format.
    /// Translates to `#[serde(rename = "...")]`.
    pub rename: Option<String>,
    /// Is this an xml attribute?
    /// On the xml side, this translates to `#[serde(rename = "@...")]`.
    /// On the json side, this is ignored.
    pub attribute: bool,
    /// Is this field an Option<> that should be skipped if it is None?
    /// If it is then this translates to `#[serde(skip_serializing_if = "crate::deser::is_none")]`.
    pub optional: bool,
    /// Should the field be flattened
    /// If it is then this translates to `#[serde(flatten)]`.
    pub flatten: bool,
    /// Is this a choice type?
    /// If it is a choice type and the format is xml then the flatten attribute is applied and the field is renamed to `#[serde(rename="$value")]`.
    /// In json it is ignored and only the flatten attribute is used.
    pub choice: bool,
    /// Is this an xml value?
    /// If it is then this translates to `#[serde(rename="$value")]`.
    /// In json this translates to `#[serde(rename = "value")]`.
    /// This option is incompatible with the `flatten`, `choice`, `attribute` and `rename` options.
    pub value: bool,
    /// The version since this field was added.
    /// Right now this is not used.
    #[allow(unused)]
    pub since: Option<Version>,
}

impl FieldAttr {
    pub fn from_attrs(attrs: &[syn::Attribute]) -> Result<Self> {
        let metas = obtain_meta_list(attrs)?;
        let mut rename = None;
        let mut attribute = false;
        let mut optional = false;
        let mut flatten = false;
        let mut choice = false;
        let mut value = false;
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
                syn::Meta::Path(p) if OPTIONAL == p => {
                    optional = true;
                }
                syn::Meta::Path(p) if FLATTEN == p => {
                    flatten = true;
                }
                syn::Meta::Path(p) if CHOICE == p => {
                    choice = true;
                }
                syn::Meta::Path(p) if VALUE == p => {
                    value = true;
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

        Ok(Self {
            rename,
            attribute,
            optional,
            flatten,
            choice,
            value,
            since,
        })
    }

    fn apply(&self, format: Format, field: &mut syn::Field) -> Result<()> {
        let field_name = {
            let mut base_name = match &self.rename {
                Some(name) => name.clone(),
                None => util::string_to_camel_case(
                    &field
                        .ident
                        .as_ref()
                        .map(|i| i.to_string())
                        .unwrap_or_default(),
                ),
            };

            if format == Format::Xml && self.attribute {
                base_name.insert(0, '@');
            }

            base_name
        };

        if !(self.choice && format == Format::Xml || self.value) {
            field.attrs.push(syn::parse_quote! {
                #[serde(rename = #field_name)]
            });
        }

        if self.optional {
            field.attrs.push(syn::parse_quote! {
                #[serde(skip_serializing_if = "crate::deser::is_none")]
            });
        }

        if self.flatten {
            if self.choice && format == Format::Xml {
                field.attrs.push(syn::parse_quote! {
                    #[serde(rename="$value")]
                });
            } else {
                field.attrs.push(syn::parse_quote! {
                   #[serde(flatten)]
                });
            }
        }

        if self.value {
            if self.flatten || self.choice || self.attribute || self.rename.is_some() {
                return Err(syn::Error::new_spanned(
                    field,
                    "The value attribute is incompatible with the flatten, choice and attribute attributes",
                ));
            }

            if format == Format::Xml {
                field.attrs.push(syn::parse_quote! {
                    #[serde(rename="$value")]
                });
            } else {
                field.attrs.push(syn::parse_quote! {
                    #[serde(rename = "value")]
                });
            }
        }

        Ok(())
    }
}
