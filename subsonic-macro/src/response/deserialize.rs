use proc_macro2::TokenStream;
use syn::Result;

use crate::util;

use super::{
    attr,
    container::{Container, Field, Variant},
};

pub fn expand(input: &syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
    let container_attrs = attr::ContainerAttr::from_attrs(&input.attrs)?;
    let output = if container_attrs.serde {
        let container_ident = &input.ident;
        quote::quote! {
            impl<'de> crate::deser::SubsonicDeserialize<'de> for #container_ident {
                type Seed = crate::deser::AnySeed<#container_ident>;
            }
        }
    } else {
        let container = Container::from_input(input)?;
        match container.data {
            super::container::Data::Struct(ref fields) => expand_struct(&container, fields)?,
            super::container::Data::Enum(ref variants) => expand_enum(&container, variants)?,
        }
    };
    Ok(quote::quote! {
        const _: () = {
            #output
        };
    })
}

fn expand_struct(container: &Container, fields: &[Field]) -> Result<TokenStream> {
    let container_ident = &container.ident;

    let key_decls = struct_fields_key_decl(fields);
    let opt_inits = struct_fields_opt_init(fields);
    let match_arms = struct_fields_match_arms(fields);
    let flat_assigns = struct_fields_flatten_assign(fields);
    let option_unwraps = struct_fields_option_unwrap(fields);
    let fields = fields.iter().map(|f| f.ident);

    let output = quote::quote! {
        pub struct Seed(crate::common::Format, crate::common::Version);
        impl From<(crate::common::Format, crate::common::Version)> for Seed {
            fn from((format, version): (crate::common::Format, crate::common::Version)) -> Self {
                Self(format, version)
            }
        }
        impl<'de> serde::de::Visitor<'de> for Seed {
            type Value = #container_ident;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(std::stringify!(#container_ident))
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let __vformat = self.0;
                let __version = self.1;

                #(#key_decls)*

                #(#opt_inits)*

                let mut buffered = Vec::new();
                while let Some(key) = map.next_key::<String>()? {
                    match key {
                        #(#match_arms)*
                        _ => {
                            buffered.push((key, map.next_value::<crate::deser::Value>()?));
                        }
                    }
                }

                #(#flat_assigns)*

                #(#option_unwraps)*

                Ok(#container_ident {
                    #(#fields),*
                })
            }
        }
        impl<'de> serde::de::DeserializeSeed<'de> for Seed {
            type Value = #container_ident;

            fn deserialize<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                deserializer.deserialize_map(self)
            }
        }
        impl<'de> crate::deser::SubsonicDeserialize<'de> for #container_ident {
            type Seed = Seed;
        }
        impl<'de> serde::Deserialize<'de> for #container_ident {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                serde::de::DeserializeSeed::deserialize(
                    <Self as crate::deser::SubsonicDeserialize>::Seed::from((
                        crate::common::Format::Json,
                        crate::common::Version::LATEST,
                    )),
                    deserializer
                )
            }
        }
    };
    Ok(output)
}

fn struct_fields_option_unwrap(fields: &[Field]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| struct_field_option_unwrap(field))
        .collect()
}

fn struct_field_option_unwrap(field: &Field) -> TokenStream {
    let field_ident = field.ident;
    if field.attrs.optional {
        quote::quote! {
            let #field_ident = #field_ident.unwrap_or_default();
        }
    } else {
        quote::quote! {
            let #field_ident = #field_ident.ok_or_else(|| {
                serde::de::Error::missing_field(
                    std::stringify!(#field_ident),
                )
            })?;
        }
    }
}

fn struct_fields_flatten_assign(fields: &[Field]) -> Vec<TokenStream> {
    // Only 1 flattened field is supported atm
    fields
        .iter()
        .map(|field| struct_field_flatten_assign(field))
        .collect()
}

fn struct_field_flatten_assign(field: &Field) -> TokenStream {
    if !field.attrs.flatten {
        return quote::quote! {};
    }
    let field_ident = field.ident;
    let field_ty = field.ty;
    quote::quote! {
        #field_ident = Some(
            serde::de::DeserializeSeed::deserialize(
            <<#field_ty as crate::deser::SubsonicDeserialize>::Seed as From<(
                crate::common::Format,
                crate::common::Version,
            )>>::from((__vformat, __version)), crate::deser::FlatMapDeserializer::new(__vformat, buffered))
            .map_err(serde::de::Error::custom)?,
        );
    }
}

fn struct_fields_match_arms(fields: &[Field]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| struct_field_match_arm(field))
        .collect()
}

fn struct_field_match_arm(field: &Field) -> TokenStream {
    if field.attrs.flatten {
        return quote::quote! {};
    }
    let field_ty = field.ty;
    let field_ident = field.ident;
    let key_ident = struct_field_key_ident(field);
    quote::quote! {
        k if k == #key_ident => {
            #field_ident = Some(map.next_value_seed(
                <<#field_ty as crate::deser::SubsonicDeserialize>::Seed as From<(
                    crate::common::Format,
                    crate::common::Version,
                )>>::from((__vformat, __version))
            )?);
        }
    }
}

fn struct_fields_opt_init(fields: &[Field]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| struct_field_opt_init(field))
        .collect()
}

fn struct_field_opt_init(field: &Field) -> TokenStream {
    let field_ident = &field.ident;
    let field_ty = &field.ty;
    if field.attrs.flatten {
        quote::quote! {
            let mut #field_ident: Option<#field_ty>;
        }
    } else {
        quote::quote! {
            let mut #field_ident: Option<#field_ty> = None;
        }
    }
}

fn struct_fields_key_decl(fields: &[Field]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| struct_field_key_decl(field))
        .collect()
}

fn struct_field_key_decl(field: &Field) -> TokenStream {
    if field.attrs.flatten {
        return quote::quote! {};
    }

    let key_ident = struct_field_key_ident(field);
    let key = match field.attrs.rename {
        Some(ref key) => key.clone(),
        None => field.ident.to_string(),
    };

    let key_json = util::string_to_camel_case(&key);
    let key_xml = if field.attrs.value {
        "$text".to_string()
    } else {
        match field.attrs.attribute {
            true => format!("@{}", key_json),
            false => key_json.clone(),
        }
    };

    quote::quote! {
        let #key_ident = match __vformat {
            crate::common::Format::Json => #key_json,
            crate::common::Format::Xml => #key_xml,
        };
    }
}

fn struct_field_key_ident(field: &Field) -> syn::Ident {
    quote::format_ident!("__key_{}", field.ident)
}

fn expand_enum(container: &Container, variants: &[Variant]) -> Result<TokenStream> {
    let container_ident = &container.ident;
    let variant_idents = variants.iter().map(|v| v.ident);
    let variant_names = variants
        .iter()
        .map(|v| util::string_to_camel_case(&v.ident.to_string()))
        .collect::<Vec<_>>();
    let variant_types = variants.iter().map(|v| v.ty);

    let output = quote::quote! {
        pub struct Seed(crate::common::Format, crate::common::Version);
        impl From<(crate::common::Format, crate::common::Version)> for Seed {
            fn from((format, version): (crate::common::Format, crate::common::Version)) -> Self {
                Self(format, version)
            }
        }
        impl<'de> serde::de::Visitor<'de> for Seed {
            type Value = #container_ident;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(std::stringify!(#container_ident))
            }

            fn visit_enum<A>(self, data: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::EnumAccess<'de>,
            {
                use serde::de::VariantAccess;
                let __vformat = self.0;
                let __version = self.1;

                let (variant, access) = data.variant::<String>()?;
                match variant.as_str() {
                    #(
                        #variant_names => {
                            let __v = access.newtype_variant_seed(
                                <<#variant_types as crate::deser::SubsonicDeserialize>::Seed as From<(
                                    crate::common::Format,
                                    crate::common::Version,
                                )>>::from((__vformat, __version))
                            )?;
                            Ok(#container_ident::#variant_idents(__v))
                        }
                    )*
                    _ => Err(serde::de::Error::unknown_variant(&variant, &[
                        #(#variant_names),*
                    ])),
                }
            }
        }
        impl<'de> serde::de::DeserializeSeed<'de> for Seed {
            type Value = #container_ident;

            fn deserialize<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                const VARIANTS: &'static [&'static str] = &[
                    #(#variant_names),*
                ];
                deserializer.deserialize_enum(
                    std::stringify!(#container_ident),
                    VARIANTS,
                    self,
                )
            }
        }
        impl<'de> crate::deser::SubsonicDeserialize<'de> for #container_ident {
            type Seed = Seed;
        }
    };
    Ok(output)
}
