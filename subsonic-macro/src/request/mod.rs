use syn::Result;

use crate::{attr, util, version::Version};

struct ContainerAttributes {
    since: Version,
    path: String,
}

impl ContainerAttributes {
    fn extract(attrs: &mut Vec<syn::Attribute>) -> Result<Self> {
        let metas = attr::extract_meta_list(attrs)?;
        let mut since = None;
        let mut path = None;

        for meta in metas {
            match &meta {
                syn::Meta::NameValue(
                    meta @ syn::MetaNameValue {
                        lit: syn::Lit::Str(value),
                        ..
                    },
                ) if attr::SINCE == meta.path => {
                    if since.is_some() {
                        return Err(syn::Error::new_spanned(meta, "Duplicate attribute"));
                    } else {
                        let v = Version::parse(&value.value())
                            .ok_or_else(|| syn::Error::new_spanned(meta, "Invalid version"))?;
                        since = Some(v);
                    }
                }
                syn::Meta::NameValue(
                    meta @ syn::MetaNameValue {
                        lit: syn::Lit::Str(value),
                        ..
                    },
                ) if attr::PATH == meta.path => {
                    if path.is_some() {
                        return Err(syn::Error::new_spanned(meta, "Duplicate attribute"));
                    } else {
                        path = Some(value.value());
                    }
                }
                _ => return Err(syn::Error::new_spanned(meta, "Invalid subsonic attribute")),
            }
        }

        let since = since.ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "Missing since attribute on subsonic attribute",
            )
        })?;
        let path = path.ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "Missing path attribute on subsonic attribute",
            )
        })?;

        Ok(Self { since, path })
    }
}

struct FieldAttributes {
    flatten: bool,
    rename: Option<String>,
}

impl FieldAttributes {
    fn obtain(attrs: &Vec<syn::Attribute>) -> Result<Self> {
        let metas = attr::obtain_meta_list(attrs)?;
        let mut flatten = false;
        let mut rename = None;

        for meta in metas {
            match &meta {
                syn::Meta::Path(path) if attr::FLATTEN == path => {
                    if flatten {
                        return Err(syn::Error::new_spanned(path, "Duplicate flatten attribute"));
                    } else {
                        flatten = true;
                    }
                }
                syn::Meta::NameValue(
                    meta @ syn::MetaNameValue {
                        lit: syn::Lit::Str(value),
                        ..
                    },
                ) if attr::RENAME == meta.path => {
                    if rename.is_some() {
                        return Err(syn::Error::new_spanned(meta, "Duplicate attribute"));
                    } else {
                        rename = Some(value.value());
                    }
                }
                _ => return Err(syn::Error::new_spanned(meta, "Invalid subsonic attribute")),
            }
        }

        Ok(Self { flatten, rename })
    }
}

fn input_get_data_struct(input: &syn::DeriveInput) -> Result<&syn::DataStruct> {
    match &input.data {
        syn::Data::Struct(data) => Ok(data),
        _ => Err(syn::Error::new_spanned(
            input,
            "Only structs can be used with #[derive(SubsonicRequest)]",
        )),
    }
}

fn field_get_name_str(field: &syn::Field) -> Result<String> {
    let camel_case = field
        .ident
        .as_ref()
        .map(|i| i.to_string())
        .as_deref()
        .map(util::string_to_camel_case)
        .ok_or_else(|| syn::Error::new_spanned(field, "Field must be named"));
    let attrs = FieldAttributes::obtain(&field.attrs)?;
    if let Some(rename) = attrs.rename {
        Ok(rename)
    } else {
        camel_case
    }
}

pub fn expand(mut input: syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
    let container_attrs = ContainerAttributes::extract(&mut input.attrs)?;
    let container_ident = &input.ident;

    let path = &container_attrs.path;
    let since = &container_attrs.since;
    let output = quote::quote! {
        impl crate::request::SubsonicRequest for #container_ident {
            const PATH: &'static str = #path;
            const SINCE: crate::common::Version = #since;
        }
    };

    Ok(output)
}
