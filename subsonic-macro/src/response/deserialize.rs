use proc_macro2::TokenStream;
use quote::ToTokens;
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

    let (impl_t, type_t, where_t) = deserialize_generics(container);
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
            type Value = #container_ident #type_t;

            fn deserialize<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                deserializer.deserialize_map(self)
            }
        }
        impl #impl_t crate::deser::SubsonicDeserialize<'de> for #container_ident #type_t #where_t {
            type Seed = Seed;
        }
        impl #impl_t serde::Deserialize<'de> for #container_ident #type_t #where_t {
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

/// Returns a tuple of (impl, type, where) generics
fn deserialize_generics(container: &Container) -> (TokenStream, TokenStream, TokenStream) {
    let mut generics_with_de = container.generics.clone();
    generics_with_de.params.insert(0, syn::parse_quote!('de));
    let (impl_de, _, _) = generics_with_de.split_for_impl();
    let (_, ty_de, where_clause) = container.generics.split_for_impl();

    let mut impl_tokens = TokenStream::new();
    impl_de.to_tokens(&mut impl_tokens);

    let mut ty_tokens = TokenStream::new();
    ty_de.to_tokens(&mut ty_tokens);

    let mut where_tokens = TokenStream::new();
    where_clause.to_tokens(&mut where_tokens);

    (impl_tokens, ty_tokens, where_tokens)
}

fn struct_fields_option_unwrap(fields: &[Field]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| struct_field_option_unwrap(field))
        .collect()
}

fn struct_field_option_unwrap(field: &Field) -> TokenStream {
    let field_ident = field.ident;
    if util::type_is_vec(field.ty) || util::type_is_option(field.ty) {
        quote::quote! {
            let #field_ident = #field_ident.unwrap_or_default();
        }
    } else {
        let mut tokens = quote::quote! {
            let #field_ident = #field_ident.ok_or_else(|| {
                serde::de::Error::missing_field(
                    std::stringify!(#field_ident),
                )
            })?;
        };
        if let Some(since) = field.attrs.since {
            tokens = quote::quote! {
                let #field_ident = if __version >= #since {
                    #tokens
                    #field_ident
                } else {
                    #field_ident.unwrap_or_default()
                };
            };
        }
        tokens
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
    let (key_json, key_xml) = if let Some(ref rename) = field.attrs.rename {
        let key_json = rename.clone();
        let key_xml = if field.attrs.attribute {
            format!("@{}", key_json)
        } else {
            key_json.clone()
        };
        (key_json, key_xml)
    } else {
        let key = field.ident.to_string();
        let key_json = util::string_to_camel_case(&key);
        let key_xml = if field.attrs.value {
            "$text".to_string()
        } else {
            match field.attrs.attribute {
                true => format!("@{}", key_json),
                false => key_json.clone(),
            }
        };
        (key_json, key_xml)
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
    let variant_names = variants
        .iter()
        .map(|v| util::string_to_camel_case(&v.ident.to_string()))
        .collect::<Vec<_>>();
    let match_arms = enum_variants_match_arm(container, variants);

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

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>
            {
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        #(#match_arms)*
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(#container_ident::Empty)
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

fn enum_variants_match_arm(container: &Container, variants: &[Variant]) -> Vec<TokenStream> {
    variants
        .iter()
        .map(|variant| enum_variant_match_arm(container, variant))
        .collect()
}

fn enum_variant_match_arm(container: &Container, variant: &Variant) -> TokenStream {
    let container_ident = container.ident;
    let variant_ident = variant.ident;
    let variant_name = util::string_to_camel_case(&variant_ident.to_string());
    let variant_ty = variant.ty;
    quote::quote! {
        #variant_name => {
            let __v = map.next_value_seed(
                <<#variant_ty as crate::deser::SubsonicDeserialize>::Seed as From<(
                    crate::common::Format,
                    crate::common::Version,
                )>>::from((self.0, self.1))
            )?;
            return Ok(#container_ident::#variant_ident(__v));
        }
    }
}
