use syn::Result;

use crate::{attr, version::Version};

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

pub fn expand(mut input: syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
    let container_attrs = ContainerAttributes::extract(&mut input.attrs)?;
    let container_ident = &input.ident;

    let path = format!("/rest/{}", container_attrs.path);
    let since = &container_attrs.since;
    let output = quote::quote! {
        impl crate::request::SubsonicRequest for #container_ident {
            const PATH: &'static str = #path;
            const SINCE: crate::common::Version = #since;
        }
    };

    Ok(output)
}
