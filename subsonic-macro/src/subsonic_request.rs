use crate::attr;

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
        let metas = attr::extract_named_meta("subsonic", attrs)?;
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

struct FieldAttributes {
    flatten: bool,
}

impl FieldAttributes {
    fn obtain(attrs: &Vec<syn::Attribute>) -> Result<Self> {
        let metas = attr::obtain_meta_list(attrs)?;
        let mut flatten = false;

        for meta in metas {
            match meta {
                syn::Meta::Path(path) if attr::FLATTEN == path => {
                    if flatten {
                        return Err(syn::Error::new_spanned(path, "Duplicate flatten attribute"));
                    } else {
                        flatten = true;
                    }
                }
                _ => return Err(syn::Error::new_spanned(meta, "Invalid subsonic attribute")),
            }
        }

        Ok(Self { flatten })
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

fn expand_to_query(input: syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
    let data = input_get_data_struct(&input)?;
    let ident = &input.ident;

    // The statements to build each field in the query
    // Example for a regular field:
    // <u32 as crate::query::ToQueryValue>::to_query_builder(&self.id, builder, "id");
    // Example for a flattened field:
    // <Foo as crate::query::ToQuery>::to_query_builder(&self.foo, builder);
    let mut query_build_stmts = Vec::new();
    for field in data.fields.iter() {
        let field_ty = &field.ty;
        let field_ident = &field.ident;
        let field_ident_str = field_ident
            .as_ref()
            .expect("Field must be named")
            .to_string();

        let attrs = FieldAttributes::obtain(&field.attrs)?;
        let stmt = if attrs.flatten {
            quote::quote! {
                <#field_ty as crate::query::ToQuery>::to_query_builder(&self.#field_ident, builder);
            }
        } else {
            quote::quote! {
                <#field_ty as crate::query::ToQueryValue>::to_query_builder(&self.#field_ident, builder, #field_ident_str);
            }
        };
        query_build_stmts.push(stmt);
    }

    let output = quote::quote! {
        impl crate::query::ToQuery for #ident {
            fn to_query_builder<B>(&self, builder: &mut B)
            where
                B: crate::query::QueryBuilder,
            {
                #(#query_build_stmts)*
            }
        }
    };
    Ok(output)
}

fn expand_from_query(input: syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
    let output = quote::quote! {};
    Ok(output)
}

pub fn expand(mut input: syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
    let impl_to_query = expand_to_query(input.clone())?;
    let impl_from_query = expand_from_query(input.clone())?;

    let container_attrs = ContainerAttributes::extract(&mut input.attrs)?;
    let container_ident = &input.ident;

    let path = &container_attrs.path;
    let since = &container_attrs.since;
    let output = quote::quote! {
        impl crate::request::SubsonicRequest for #container_ident {
            const PATH: &'static str = #path;
            const SINCE: crate::common::Version = #since;
        }

        #impl_to_query
        #impl_from_query
    };

    Ok(output)
}
