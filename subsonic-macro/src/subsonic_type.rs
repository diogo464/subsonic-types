type Result<T, E = syn::Error> = std::result::Result<T, E>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Format {
    Json,
    Xml,
}

struct Attributes {
    rename: Option<String>,
    attribute: bool,
    optional: bool,
    flatten: bool,
}

impl Attributes {
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

        let mut rename = None;
        let mut attribute = false;
        let mut optional = false;
        let mut flatten = false;

        for meta in metas {
            match &meta {
                syn::Meta::List(list @ syn::MetaList { nested, .. })
                    if list.path.is_ident("subsonic") =>
                {
                    for n in nested {
                        match n {
                            syn::NestedMeta::Meta(syn::Meta::NameValue(nv))
                                if nv.path.is_ident("rename") =>
                            {
                                if let syn::Lit::Str(s) = &nv.lit {
                                    rename = Some(s.value());
                                }
                            }
                            syn::NestedMeta::Meta(syn::Meta::Path(p))
                                if p.is_ident("attribute") =>
                            {
                                attribute = true;
                            }
                            syn::NestedMeta::Meta(syn::Meta::Path(p)) if p.is_ident("optional") => {
                                optional = true;
                            }
                            syn::NestedMeta::Meta(syn::Meta::Path(p)) if p.is_ident("flatten") => {
                                flatten = true;
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

        Ok(Self {
            rename,
            attribute,
            optional,
            flatten,
        })
    }

    fn apply(&self, field: &mut syn::Field) -> Result<()> {
        let field_name = {
            let mut base_name = match &self.rename {
                Some(name) => name.clone(),
                None => string_to_camel_case(
                    &field
                        .ident
                        .as_ref()
                        .map(|i| i.to_string())
                        .unwrap_or_default(),
                ),
            };

            if self.attribute {
                base_name.insert(0, '@');
            }

            base_name
        };

        field.attrs.push(syn::parse_quote! {
            #[serde(rename = #field_name)]
        });

        if self.optional {
            field.attrs.push(syn::parse_quote! {
                #[serde(skip_serializing_if = "Option::is_none")]
            });
        }

        if self.flatten {
            field.attrs.push(syn::parse_quote! {
                #[serde(flatten)]
            });
        }

        Ok(())
    }
}

fn apply_field_attributes(field: &mut syn::Field) -> Result<()> {
    let attrs = Attributes::extract(&mut field.attrs)?;
    attrs.apply(field)
}

fn input_apply_field_attributes(input: &mut syn::DeriveInput) -> Result<()> {
    match &mut input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(named),
            ..
        }) => {
            for field in named.named.iter_mut() {
                apply_field_attributes(field)?;
            }
        }
        _ => unimplemented!("Only structs are supported"),
    }
    Ok(())
}

struct SerializeOutput {
    patched_input: syn::DeriveInput,
    from_impl: proc_macro2::TokenStream,
}

struct SerializeBuilder<'a> {
    input: &'a syn::DeriveInput,
    se_lifetime: &'a syn::Lifetime,
    format: Format,
}

impl<'a> SerializeBuilder<'a> {
    fn new(input: &'a syn::DeriveInput, se_lifetime: &'a syn::Lifetime, format: Format) -> Self {
        Self {
            input,
            se_lifetime,
            format,
        }
    }

    fn build(self) -> Result<SerializeOutput> {
        let patched_input = self.patched_input()?;
        let from_impl = self.impl_from(&patched_input)?;

        Ok(SerializeOutput {
            patched_input,
            from_impl,
        })
    }

    fn patched_input(&self) -> Result<syn::DeriveInput> {
        let mut patched = self.input.clone();

        self.patch_ident(&mut patched);
        self.append_derive_serialize(&mut patched);
        self.append_se_lifetime(&mut patched);
        self.patch_field_types(&mut patched);
        self.append_phatom_field(&mut patched);
        input_apply_field_attributes(&mut patched)?;

        Ok(patched)
    }

    fn impl_from(&self, patched: &syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
        let input_ident = &self.input.ident;
        let patched_ident = &patched.ident;
        let se_lifetime = self.se_lifetime;

        let output = match &self.input.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(named),
                ..
            }) => {
                let field_idents = named.named.iter().map(|f| &f.ident);

                quote::quote! {
                    impl<#se_lifetime> From<&#se_lifetime #input_ident> for #patched_ident<#se_lifetime> {
                        fn from(value: &#se_lifetime #input_ident) -> Self {
                            Self {
                                #(
                                    #field_idents: From::from(&value.#field_idents),
                                )*
                                __subsonice_phantom: std::marker::PhantomData,
                            }
                        }
                    }
                }
            }
            _ => unimplemented!("Only structs with named fields are supported for now"),
        };

        Ok(output)
    }

    fn patch_ident(&self, patched: &mut syn::DeriveInput) {
        patched.ident = syn::Ident::new(
            match self.format {
                Format::Json => "ToJson",
                Format::Xml => "ToXml",
            },
            proc_macro2::Span::call_site(),
        );
    }

    fn append_derive_serialize(&self, patched: &mut syn::DeriveInput) {
        patched
            .attrs
            .push(syn::parse_quote!(#[derive(serde::Serialize)]));
    }

    fn append_se_lifetime(&self, patched: &mut syn::DeriveInput) {
        let lifetime = self.se_lifetime;
        patched
            .generics
            .params
            .insert(0, syn::parse_quote!(#lifetime));
    }

    fn patch_field_types(&self, patched: &mut syn::DeriveInput) {
        let lifetime = self.se_lifetime;
        let assoc_ty = syn::Ident::new(
            match self.format {
                Format::Json => "ToJson",
                Format::Xml => "ToXml",
            },
            proc_macro2::Span::call_site(),
        );

        match &mut patched.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(fields),
                ..
            }) => {
                for field in &mut fields.named {
                    let ty = &mut field.ty;
                    *ty =
                        syn::parse_quote!(<#ty as crate::SubsonicSerialize<#lifetime>>::#assoc_ty);
                }
            }
            _ => unimplemented!("Only structs with named fields are supported for now"),
        }
    }

    fn append_phatom_field(&self, patched: &mut syn::DeriveInput) {
        let lifetime = self.se_lifetime;

        match &mut patched.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(fields),
                ..
            }) => {
                fields.named.push(syn::Field {
                    attrs: vec![syn::parse_quote!(#[serde(skip)])],
                    vis: syn::Visibility::Inherited,
                    ident: syn::parse_quote!(__subsonice_phantom),
                    colon_token: Some(syn::Token![:](proc_macro2::Span::call_site())),
                    ty: syn::parse_quote!(std::marker::PhantomData<&#lifetime ()>),
                });
            }
            _ => unimplemented!("Only structs with named fields are supported for now"),
        }
    }
}

struct DeserializeOutput {
    patched_input: syn::DeriveInput,
    into_impl: proc_macro2::TokenStream,
}

struct DeserializeBuilder<'a> {
    input: &'a syn::DeriveInput,
    format: Format,
}

impl<'a> DeserializeBuilder<'a> {
    fn new(input: &'a syn::DeriveInput, format: Format) -> Self {
        Self { input, format }
    }

    fn build(self) -> Result<DeserializeOutput> {
        let patched_input = self.patched_input()?;
        let into_impl = self.impl_into(&patched_input)?;

        Ok(DeserializeOutput {
            patched_input,
            into_impl,
        })
    }

    fn patched_input(&self) -> Result<syn::DeriveInput> {
        let mut patched = self.input.clone();

        self.patch_ident(&mut patched);
        self.append_derive_deserialize(&mut patched);
        self.patch_field_types(&mut patched);
        input_apply_field_attributes(&mut patched)?;

        Ok(patched)
    }

    fn impl_into(&self, patched: &syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
        let input_ident = &self.input.ident;
        let patched_ident = &patched.ident;

        let output = match &self.input.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(named),
                ..
            }) => {
                let field_idents = named.named.iter().map(|f| &f.ident);

                quote::quote! {
                    impl Into<#input_ident> for #patched_ident {
                        fn into(self) -> #input_ident {
                            #input_ident {
                                #(
                                    #field_idents: Into::into(self.#field_idents),
                                )*
                            }
                        }
                    }
                }
            }
            _ => unimplemented!("Only structs with named fields are supported for now"),
        };

        Ok(output)
    }

    fn patch_ident(&self, patched: &mut syn::DeriveInput) {
        patched.ident = syn::Ident::new(
            match self.format {
                Format::Json => "FromJson",
                Format::Xml => "FromXml",
            },
            proc_macro2::Span::call_site(),
        );
    }

    fn append_derive_deserialize(&self, patched: &mut syn::DeriveInput) {
        patched
            .attrs
            .push(syn::parse_quote!(#[derive(serde::Deserialize)]));
    }

    fn patch_field_types(&self, patched: &mut syn::DeriveInput) {
        let assoc_ty = syn::Ident::new(
            match self.format {
                Format::Json => "FromJson",
                Format::Xml => "FromXml",
            },
            proc_macro2::Span::call_site(),
        );

        match &mut patched.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(fields),
                ..
            }) => {
                for field in &mut fields.named {
                    let ty = &mut field.ty;
                    *ty = syn::parse_quote!(<#ty as crate::SubsonicDeserialize>::#assoc_ty);
                }
            }
            _ => unimplemented!("Only structs with named fields are supported for now"),
        }
    }
}

pub fn expand(input: syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
    let input_ident = &input.ident;
    let se_lifetime = syn::Lifetime::new("'__subsonice_se", proc_macro2::Span::call_site());

    let se_json = SerializeBuilder::new(&input, &se_lifetime, Format::Json).build()?;
    let se_json_patched = &se_json.patched_input;
    let se_json_impl = &se_json.from_impl;
    let se_json_ident = &se_json_patched.ident;

    let de_json = DeserializeBuilder::new(&input, Format::Json).build()?;
    let de_json_patched = &de_json.patched_input;
    let de_json_impl = &de_json.into_impl;
    let de_json_ident = &de_json_patched.ident;

    let se_xml = SerializeBuilder::new(&input, &se_lifetime, Format::Xml).build()?;
    let se_xml_patched = &se_xml.patched_input;
    let se_xml_impl = &se_xml.from_impl;
    let se_xml_ident = &se_xml_patched.ident;

    let de_xml = DeserializeBuilder::new(&input, Format::Xml).build()?;
    let de_xml_patched = &de_xml.patched_input;
    let de_xml_impl = &de_xml.into_impl;
    let de_xml_ident = &de_xml_patched.ident;

    let output = quote::quote! {
        const _: () = {
            #se_json_patched
            #se_json_impl
            #de_json_patched
            #de_json_impl

            #se_xml_patched
            #se_xml_impl
            #de_xml_patched
            #de_xml_impl

            impl<#se_lifetime> crate::SubsonicSerialize<#se_lifetime> for #input_ident {
                type ToJson = #se_json_ident<#se_lifetime>;
                type ToXml = #se_xml_ident<#se_lifetime>;
            }

            impl crate::SubsonicDeserialize for #input_ident {
                type FromJson = #de_json_ident;
                type FromXml = #de_xml_ident;
            }
        };
    };
    Ok(output)
}

fn string_to_camel_case(string: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    for c in string.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_case() {
        assert_eq!(string_to_camel_case("foo_bar"), "fooBar");
        assert_eq!(string_to_camel_case("foo_bar_baz"), "fooBarBaz");
        assert_eq!(string_to_camel_case("foo_bar_baz_qux"), "fooBarBazQux");

        assert_eq!(string_to_camel_case("foo"), "foo");
        assert_eq!(string_to_camel_case("foo_bar"), "fooBar");
    }
}
