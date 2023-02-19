type Result<T, E = syn::Error> = std::result::Result<T, E>;

struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Version {
    fn parse(v: &str) -> Option<Self> {
        let mut parts = v.split('.');
        let major = parts.next()?.parse().ok()?;
        let minor = parts.next()?.parse().ok()?;
        let patch = parts.next()?.parse().ok()?;
        Some(Self {
            major,
            minor,
            patch,
        })
    }
}

impl quote::ToTokens for Version {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let major = self.major;
        let minor = self.minor;
        let patch = self.patch;
        quote::quote! {
            crate::common::Version::new(#major, #minor, #patch)
        }
        .to_tokens(tokens)
    }
}

struct ContainerAttributes {
    since: Version,
    path: String,
}

impl ContainerAttributes {
    fn extract(attrs: &mut Vec<syn::Attribute>) -> Result<Self> {
        let mut extracted = Vec::new();
        let mut index = 0;
        while index < attrs.len() {
            let attr = &attrs[index];
            if attr.path.is_ident("subsonic") {
                extracted.push(attr.clone());
                attrs.remove(index);
            } else {
                index += 1;
            }
        }
        Self::from_attrs(&extracted)
    }

    fn from_attrs(attrs: &[syn::Attribute]) -> Result<Self> {
        let mut metas = Vec::new();
        for attr in attrs {
            metas.push(attr.parse_meta()?);
        }

        let mut since = None;
        let mut path = None;

        for meta in metas {
            match &meta {
                syn::Meta::List(list @ syn::MetaList { nested, .. })
                    if list.path.is_ident("subsonic") =>
                {
                    for n in nested {
                        match n {
                            syn::NestedMeta::Meta(syn::Meta::NameValue(
                                nv @ syn::MetaNameValue {
                                    lit: syn::Lit::Str(value),
                                    ..
                                },
                            )) if nv.path.is_ident("since") => {
                                if since.is_some() {
                                    return Err(syn::Error::new_spanned(
                                        nv,
                                        "Duplicate since attribute",
                                    ));
                                } else {
                                    let v = Version::parse(&value.value()).ok_or_else(|| {
                                        syn::Error::new_spanned(nv, "Invalid version")
                                    })?;
                                    since = Some(v);
                                }
                            }
                            syn::NestedMeta::Meta(syn::Meta::NameValue(
                                nv @ syn::MetaNameValue {
                                    lit: syn::Lit::Str(value),
                                    ..
                                },
                            )) if nv.path.is_ident("path") => {
                                if path.is_some() {
                                    return Err(syn::Error::new_spanned(
                                        nv,
                                        "Duplicate path attribute",
                                    ));
                                } else {
                                    path = Some(value.value());
                                }
                            }
                            _ => {
                                return Err(syn::Error::new_spanned(
                                    n,
                                    "Invalid subsonic attribute",
                                ))
                            }
                        }
                    }
                }
                _ => continue,
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
