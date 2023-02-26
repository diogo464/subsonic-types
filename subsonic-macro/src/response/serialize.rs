use proc_macro2::TokenStream;
use syn::Result;

use crate::util;

use super::{
    attr,
    container::{Container, Field, Variant},
};

pub fn expand(input: &syn::DeriveInput) -> Result<TokenStream> {
    let container_attrs = attr::ContainerAttr::from_attrs(&input.attrs)?;
    let output = if container_attrs.serde {
        let container_ident = &input.ident;
        quote::quote! {
            impl crate::deser::SubsonicSerialize for #container_ident {
                fn serialize<S>(&self, serializer: S, _: crate::common::Format, _: crate::common::Version) -> std::result::Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    <Self as serde::Serialize>::serialize(self, serializer)
                }
            }
        }
    } else {
        let container = Container::from_input(input)?;
        match container.data {
            super::container::Data::Struct(ref fields) => expand_struct(&container, fields)?,
            super::container::Data::Enum(ref variants) => expand_enum(&container, variants)?,
        }
    };
    Ok(output)
}

fn expand_struct(container: &Container, fields: &[Field]) -> Result<TokenStream> {
    let container_ident = &container.ident;

    let field_count = fields.len();
    let field_key_decls = struct_fields_key_decl(fields);
    let field_key_entries = struct_fields_serialize_entry(fields);

    let output = quote::quote! {
        impl crate::deser::SubsonicSerialize for #container_ident {
            fn serialize<S>(&self, serializer: S, format: crate::common::Format, version: crate::common::Version) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::{Serializer, ser::SerializeMap};

                #(#field_key_decls)*

                let mut map = serializer.serialize_map(Some(#field_count))?;

                #(#field_key_entries)*

                map.end()
            }
        }

        impl serde::Serialize for #container_ident {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                <Self as crate::deser::SubsonicSerialize>::serialize(self, serializer, crate::common::Format::Json, crate::common::Version::LATEST)
            }
        }
    };

    Ok(output)
}

fn struct_fields_serialize_entry(fields: &[Field]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| struct_field_serialize_entry(field))
        .collect()
}

fn struct_field_serialize_entry(field: &Field) -> TokenStream {
    let field_ty = field.ty;
    let field_ident = field.ident;
    let key_ident = struct_field_key_ident(field);
    let cond = match field.attrs.since {
        Some(ref since) => quote::quote! { version >= #since },
        None => quote::quote! { true },
    };
    let output = if field.attrs.flatten {
        quote::quote! {
            <#field_ty as crate::deser::SubsonicSerialize>::serialize(
                &self.#field_ident,
                serde::__private::ser::FlatMapSerializer(&mut map),
                format,
                version,
            )?;
        }
    } else {
        quote::quote! {
            if #cond {
                map.serialize_entry(
                    #key_ident,
                    &crate::deser::SubsonicSerializeWrapper(&self.#field_ident, format, version)
                )?;
            }
        }
    };
    if field.attrs.optional {
        quote::quote! {
            if  self.#field_ident.is_some() {
                #output
            }
        }
    } else {
        output
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
            true => format!("@{}", key),
            false => key_json.clone(),
        }
    };

    quote::quote! {
        let #key_ident = match format {
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

    let variants_ident = variants.iter().map(|v| &v.ident);
    let variants_name = variants
        .iter()
        .map(|v| util::string_to_camel_case(&v.ident.to_string()));

    let output = quote::quote! {
        impl crate::deser::SubsonicSerialize for #container_ident {
            fn serialize<S>(&self, serializer: S, format: crate::common::Format, version: crate::common::Version) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::{Serializer, ser::SerializeMap};

                let mut map = serializer.serialize_map(Some(1))?;
                match self {
                    #(
                        Self::#variants_ident(v) => {
                            map.serialize_entry(
                                #variants_name,
                                &crate::deser::SubsonicSerializeWrapper(v, format, version)
                            )?;
                        },
                    )*
                }
                map.end()
            }
        }
    };
    Ok(output)
}
