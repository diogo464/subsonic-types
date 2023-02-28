use syn::Result;

use crate::{attr, util, version::Format};

fn apply_field_attributes(format: Format, field: &mut syn::Field) -> Result<()> {
    let attrs = Attributes::extract(&mut field.attrs)?;
    attrs.apply(format, field)
}

fn input_apply_field_attributes(format: Format, input: &mut syn::DeriveInput) -> Result<()> {
    match &mut input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(named),
            ..
        }) => {
            for field in named.named.iter_mut() {
                apply_field_attributes(format, field)?;
            }
        }
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            for variant in variants {
                for field in variant.fields.iter_mut() {
                    apply_field_attributes(format, field)?;
                }
            }
        }
        _ => unimplemented!("Only structs and enum are supported"),
    }
    Ok(())
}

fn serde_wrapper(format: Format) -> syn::Path {
    let ident = syn::Ident::new(
        match format {
            Format::Json => "Json",
            Format::Xml => "Xml",
        },
        proc_macro2::Span::call_site(),
    );
    syn::parse_quote!(crate::deser::#ident)
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
        // enum_insert_untagged(&mut patched);
        self.append_se_lifetime(&mut patched);
        self.patch_field_types(&mut patched);
        self.append_phatom_field(&mut patched);
        input_apply_field_attributes(self.format, &mut patched)?;

        Ok(patched)
    }

    fn impl_from(&self, patched: &syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
        let input_ident = &self.input.ident;
        let patched_ident = &patched.ident;
        let se_lifetime = self.se_lifetime;

        let method_inner = match &self.input.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(named),
                ..
            }) => {
                let field_idents = named.named.iter().map(|f| &f.ident);

                quote::quote! {
                    Self {
                        #(
                            #field_idents: From::from(&value.#field_idents),
                        )*
                        __subsonic_phantom: std::marker::PhantomData,
                    }
                }
            }
            syn::Data::Enum(syn::DataEnum { variants, .. }) => {
                let variant_idents = variants.iter().map(|v| &v.ident);

                quote::quote! {
                    match value {
                        #(
                            #input_ident::#variant_idents(v) => Self::#variant_idents(From::from(v)),
                        )*
                    }
                }
            }
            _ => unimplemented!(
                "Only structs with named fields and limited enums are supported for now"
            ),
        };

        let output = quote::quote! {
            impl<#se_lifetime> From<&#se_lifetime #input_ident> for #patched_ident<#se_lifetime> {
                fn from(value: &#se_lifetime #input_ident) -> Self {
                    #method_inner
                }
            }
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
        patched
            .attrs
            .push(syn::parse_quote!(#[serde(rename_all = "camelCase")]));
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

        let patch_field = |field: &mut syn::Field| {
            let ty = &mut field.ty;
            *ty = syn::parse_quote!(<#ty as crate::deser::SubsonicSerialize<#lifetime>>::Output);
        };

        match &mut patched.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(fields),
                ..
            }) => {
                fields.named.iter_mut().for_each(patch_field);
            }
            syn::Data::Enum(syn::DataEnum { variants, .. }) => {
                variants
                    .iter_mut()
                    .flat_map(|v| v.fields.iter_mut())
                    .for_each(patch_field);
            }
            _ => unimplemented!(
                "Only structs with named fields and limited enums are supported for now"
            ),
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
                    ident: syn::parse_quote!(__subsonic_phantom),
                    colon_token: Some(syn::Token![:](proc_macro2::Span::call_site())),
                    ty: syn::parse_quote!(std::marker::PhantomData<&#lifetime ()>),
                });
            }
            syn::Data::Enum(syn::DataEnum { variants, .. }) => {
                variants.push(syn::Variant {
                    ident: syn::Ident::new("__subsonic_phantom", proc_macro2::Span::call_site()),
                    fields: syn::Fields::Unnamed(
                        syn::parse_quote! {(std::marker::PhantomData<&#lifetime ()>)},
                    ),
                    attrs: Default::default(),
                    discriminant: None,
                });
            }
            _ => unimplemented!("Only structs with named fields and enums are supported for now"),
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
        // enum_insert_untagged(&mut patched);
        self.patch_field_types(&mut patched);
        input_apply_field_attributes(self.format, &mut patched)?;

        Ok(patched)
    }

    fn impl_into(&self, patched: &syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
        let input_ident = &self.input.ident;
        let patched_ident = &patched.ident;
        let wrapper = serde_wrapper(self.format);

        let inner = match &self.input.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(named),
                ..
            }) => {
                let field_idents = named.named.iter().map(|f| &f.ident);

                quote::quote! {
                    #input_ident {
                        #(
                            #field_idents: #wrapper::into_inner(self.#field_idents),
                        )*
                    }
                }
            }
            syn::Data::Enum(syn::DataEnum { variants, .. }) => {
                let variant_idents = variants.iter().map(|v| &v.ident);

                quote::quote! {
                    match self {
                        #(
                            #patched_ident::#variant_idents(v) => #input_ident::#variant_idents(#wrapper::into_inner(v)),
                        )*
                    }
                }
            }
            _ => unimplemented!(
                "Only structs with named fields and limited enums are supported for now"
            ),
        };

        let output = quote::quote! {
            impl Into<#input_ident> for #patched_ident {
                fn into(self) -> #input_ident {
                    #inner
                }
            }
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
        patched
            .attrs
            .push(syn::parse_quote!(#[serde(rename_all = "camelCase")]));
    }

    fn patch_field_types(&self, patched: &mut syn::DeriveInput) {
        let wrapper = serde_wrapper(self.format);
        let patch_field = |field: &mut syn::Field| {
            let ty = &mut field.ty;
            *ty = syn::parse_quote!(#wrapper<#ty>);
        };

        match &mut patched.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(fields),
                ..
            }) => {
                fields.named.iter_mut().for_each(patch_field);
            }
            syn::Data::Enum(syn::DataEnum { variants, .. }) => {
                variants
                    .iter_mut()
                    .flat_map(|v| v.fields.iter_mut())
                    .for_each(patch_field);
            }
            _ => unimplemented!(
                "Only structs with named fields and limited enums are supported for now"
            ),
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
            use serde::Deserialize;
            use serde::Serialize;

            #se_json_patched
            #se_json_impl
            #de_json_patched
            #de_json_impl

            #se_xml_patched
            #se_xml_impl
            #de_xml_patched
            #de_xml_impl

            #[automatically_derived]
            impl crate::deser::SubsonicSerialize for #input_ident {
                fn serialize<S>(
                    &self,
                    serializer: S,
                    format: crate::deser::Format,
                ) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    match format {
                        crate::deser::Format::Json => {
                            let value = #se_json_ident::from(self);
                            value.serialize(serializer)
                        }
                        crate::deser::Format::Xml => {
                            let value = #se_xml_ident::from(self);
                            value.serialize(serializer)
                        }
                    }
                }
            }

            #[automatically_derived]
            impl<'de> crate::deser::SubsonicDeserialize<'de> for #input_ident {
                fn deserialize<D>(
                    deserializer: D,
                    format: crate::deser::Format,
                ) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    match format {
                        crate::deser::Format::Json => {
                            let value = #de_json_ident::deserialize(deserializer)?;
                            Ok(value.into())
                        }
                        crate::deser::Format::Xml => {
                            let value = #de_xml_ident::deserialize(deserializer)?;
                            Ok(value.into())
                        }
                    }
                }
            }
        };
    };
    Ok(output)
}
