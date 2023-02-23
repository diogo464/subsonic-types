use proc_macro2::TokenStream;
use syn::Result;

use crate::util;

use super::{
    container::{Container, Field, Variant},
    format::Format,
};

struct Context {}

struct DeField {
    ident: syn::Ident,
    ty: syn::Type,
    attrs: Vec<syn::Attribute>,
}

pub fn expand(input: &syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
    let container = Container::from_input(input)?;
    let context = Context::new();

    let output = match container.data {
        super::container::Data::Struct(ref fields) => expand_struct(&context, &container, fields)?,
        super::container::Data::Enum(ref variants) => expand_enum(&context, &container, variants)?,
    };

    Ok(Default::default())
}

impl Context {
    fn new() -> Self {
        Self {}
    }
}

fn expand_struct(
    context: &Context,
    container: &Container,
    fields: &[Field],
) -> Result<TokenStream> {
    let json_struct = expand_struct_json(&context, &container, &fields)?;
    let xml_struct = expand_struct_xml(&context, &container, &fields)?;
    let deserialize_impl = expand_struct_impl_deserialize(&context, &container, fields)?;

    let output = quote::quote! {
        const _: () = {
            #json_struct
            #xml_struct
            #deserialize_impl
        };
    };
    Ok(output)
}

fn expand_struct_json(
    context: &Context,
    container: &Container,
    fields: &[Field],
) -> Result<TokenStream> {
    let container_ident = &container.ident;

    let fields_de = struct_fields_to_de_fields(context, Format::Json, fields);
    let fields_attr = fields_de.iter().map(|f| &f.attrs).collect::<Vec<_>>();
    let fields_ident = fields_de.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let fields_ty = fields_de.iter().map(|f| &f.ty).collect::<Vec<_>>();

    let mut fields_assign = Vec::new();
    for field in fields {
        let ident = &field.ident;
        if let Some(since) = field.attrs.since {
            if !field.attrs.optional {
                fields_assign.push(quote::quote! {
                    let #ident = if version >= #since {
                        match deserialized.#ident {
                            Some(v) => v,
                            None => return Err(serde::de::Error::missing_field(stringify!(#ident))),
                        }
                    } else {
                        deserialized.#ident.unwrap_or_default()
                    };
                });
                continue;
            }
        }
        fields_assign.push(quote::quote! { let #ident = deserialized.#ident; });
    }

    let output = quote::quote! {
        #[derive(serde::Serialize)]
        struct FromJson {
            #(
                #(#fields_attr)*
                #fields_ident: #fields_ty,
            )*
        }

        impl FromJson {
            fn deserialize_versioned<'de, D>(version: crate::common::Version, deserializer: D) -> Result<#container_ident, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let deserialized = FromJson::deserialize(deserializer)?;
                #(#fields_assign)*
                Ok(#container_ident { #(#fields_ident,)* })
            }
        }
    };

    Ok(output)
}

fn expand_struct_xml(
    context: &Context,
    container: &Container,
    fields: &[Field],
) -> Result<TokenStream> {
    Ok(Default::default())
}

fn expand_struct_impl_deserialize(
    context: &Context,
    container: &Container,
    fields: &[Field],
) -> Result<TokenStream> {
    let container_ident = &container.ident;

    let output = quote::quote! {
        impl<'de> crate::deser::SubsonicDeserialize<'de> for #container_ident {
            fn deserialize<D>(format: crate::deser::Format, version: crate::common::Version, deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>
            {
                match format {
                    crate::deser::Format::Json => FromJson::deserialize_versioned(version, deserializer),
                    crate::deser::Format::Xml => unimplemented!(),
                }
            }
        }
    };
    Ok(output)
}

fn struct_fields_to_de_fields(
    _context: &Context,
    format: Format,
    fields: &[Field],
) -> Vec<DeField> {
    let mut ser_fields = Vec::new();
    for field in fields {
        let ident = field.ident.clone();
        let attrs = struct_field_attrs(format, field);

        let ty = match field.attrs.since {
            Some(since) => {
                let field_ty = &field.ty;
                let version_code = since.as_u32();
                syn::parse_quote! { crate::deser::Versioned<#field_ty, #version_code> }
            }
            None => field.ty.clone(),
        };

        ser_fields.push(DeField { ident, ty, attrs });
    }
    ser_fields
}

fn struct_field_attrs(format: Format, field: &Field) -> Vec<syn::Attribute> {
    let mut attrs = Vec::new();
    attrs.push(struct_attr_serde_rename(format, field));
    attrs
}

fn struct_attr_serde_rename(format: Format, field: &Field) -> syn::Attribute {
    let attrs = &field.attrs;
    let field_ident = &field.ident;
    let mut name_string = if let Some(ref rename) = attrs.rename {
        util::string_to_camel_case(rename.as_str())
    } else {
        util::string_to_camel_case(&field_ident.to_string())
    };
    if field.attrs.attribute && format == Format::Xml {
        name_string = format!("@{}", name_string);
    }
    syn::parse_quote! {
        #[serde(rename = #name_string)]
    }
}

fn expand_enum(
    context: &Context,
    container: &Container,
    variants: &[Variant],
) -> Result<TokenStream> {
    Ok(Default::default())
}
