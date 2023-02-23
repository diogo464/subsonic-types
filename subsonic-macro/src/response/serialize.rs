use proc_macro2::TokenStream;
use syn::Result;

use crate::util;

use super::{
    attr::FieldAttr,
    container::{Container, Field, Variant},
    format::Format,
};

struct Context {
    ser_lifetime: syn::Lifetime,
    phatom_ident: syn::Ident,
}

struct SerField {
    ident: syn::Ident,
    ty: syn::Type,
    attrs: Vec<syn::Attribute>,
}

pub fn expand(input: &syn::DeriveInput) -> Result<TokenStream> {
    let container = Container::from_input(input)?;
    let context = Context::new();

    let output = match container.data {
        super::container::Data::Struct(ref fields) => expand_struct(&context, &container, fields)?,
        super::container::Data::Enum(ref variants) => expand_enum(&context, &container, variants)?,
    };

    Ok(output)
}

impl Context {
    fn new() -> Self {
        Self {
            ser_lifetime: syn::Lifetime::new("'__subsonic_ser", proc_macro2::Span::call_site()),
            phatom_ident: syn::Ident::new("__subsonic_phantom", proc_macro2::Span::call_site()),
        }
    }
}

fn expand_struct(
    context: &Context,
    container: &Container,
    fields: &[Field],
) -> Result<TokenStream> {
    let json_struct = expand_struct_json(&context, &container, &fields)?;
    let xml_struct = expand_struct_xml(&context, &container, &fields)?;
    let format_enum = expand_struct_format_enum(&context, &container, &fields)?;
    let impl_serialize = expand_struct_impl_serialize(&context, &container, fields)?;

    let output = quote::quote! {
        const _: () = {
            #format_enum
            #json_struct
            #xml_struct
            #impl_serialize
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
    let ser_lifetime = &context.ser_lifetime;
    let phantom_ident = &context.phatom_ident;
    let fields = struct_fields_to_ser_fields(context, Format::Json, fields);

    let fields_attr = fields.iter().map(|f| &f.attrs).collect::<Vec<_>>();
    let fields_ident = fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let fields_ty = fields.iter().map(|f| &f.ty).collect::<Vec<_>>();

    let output = quote::quote! {
        #[derive(serde::Serialize)]
        pub struct ToJson<#ser_lifetime> {
            #(
                #(#fields_attr)*
                #fields_ident: <#fields_ty as crate::deser::SubsonicSerialize<#ser_lifetime>>::Output,
            )*
            #phantom_ident: std::marker::PhantomData<&#ser_lifetime ()>,
        }

        impl<#ser_lifetime> ToJson<#ser_lifetime> {
            fn new(input: &#ser_lifetime #container_ident, version: crate::common::Version) -> Self {
                Self {
                    #(
                        #fields_ident: <#fields_ty as crate::deser::SubsonicSerialize<#ser_lifetime>>::prepare(&input.#fields_ident, crate::deser::Format::Json, version),
                    )*
                    #phantom_ident: std::marker::PhantomData,
                }
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
    let container_ident = &container.ident;
    let ser_lifetime = &context.ser_lifetime;
    let phantom_ident = &context.phatom_ident;
    let fields = struct_fields_to_ser_fields(context, Format::Xml, fields);

    let fields_attr = fields.iter().map(|f| &f.attrs).collect::<Vec<_>>();
    let fields_ident = fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let fields_ty = fields.iter().map(|f| &f.ty).collect::<Vec<_>>();

    let output = quote::quote! {
        #[derive(serde::Serialize)]
        pub struct ToXml<#ser_lifetime> {
            #(
                #(#fields_attr)*
                #fields_ident: <#fields_ty as crate::deser::SubsonicSerialize<#ser_lifetime>>::Output,
            )*
            #phantom_ident: std::marker::PhantomData<&#ser_lifetime ()>,
        }

        impl<#ser_lifetime> ToXml<#ser_lifetime> {
            fn new(input: &#ser_lifetime #container_ident, version: crate::common::Version) -> Self {
                Self {
                    #(
                        #fields_ident: <#fields_ty as crate::deser::SubsonicSerialize<#ser_lifetime>>::prepare(&input.#fields_ident, crate::deser::Format::Json, version),
                    )*
                    #phantom_ident: std::marker::PhantomData,
                }
            }
        }
    };

    Ok(output)
}

fn expand_struct_format_enum(
    context: &Context,
    container: &Container,
    _fields: &[Field],
) -> Result<TokenStream> {
    let container_ident = &container.ident;
    let ser_lifetime = &context.ser_lifetime;

    let output = quote::quote! {
        #[derive(serde::Serialize)]
        #[serde(untagged)]
        pub enum ToFormat<#ser_lifetime> {
            Json(ToJson<#ser_lifetime>),
            Xml(ToXml<#ser_lifetime>),
        }

        impl<#ser_lifetime> ToFormat<#ser_lifetime> {
            fn new(input: &#ser_lifetime #container_ident, format: crate::deser::Format, version: crate::common::Version) -> Self {
                match format {
                    crate::deser::Format::Json => Self::Json(ToJson::new(input, version)),
                    crate::deser::Format::Xml => Self::Xml(ToXml::new(input, version)),
                }
            }
        }
    };

    Ok(output)
}

fn expand_struct_impl_serialize(
    context: &Context,
    container: &Container,
    _fields: &[Field],
) -> Result<TokenStream> {
    let container_ident = &container.ident;
    let ser_lifetime = &context.ser_lifetime;

    let output = quote::quote! {
        impl<#ser_lifetime> crate::deser::SubsonicSerialize<#ser_lifetime> for #container_ident {
            type Input = &#ser_lifetime Self;
            type Output = ToFormat<#ser_lifetime>;

            fn prepare(input: Self::Input, format: crate::deser::Format, version: crate::common::Version) -> Self::Output {
                ToFormat::new(input, format, version)
            }
        }
    };

    Ok(output)
}

fn struct_fields_to_ser_fields(
    _context: &Context,
    format: Format,
    fields: &[Field],
) -> Vec<SerField> {
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

        ser_fields.push(SerField { ident, ty, attrs });
    }
    ser_fields
}

fn struct_field_attrs(format: Format, field: &Field) -> Vec<syn::Attribute> {
    let mut attrs = Vec::new();
    if field.attrs.since.is_some() {
        attrs.push(syn::parse_quote! {
            #[serde(skip_serializing_if = "crate::deser::MaybeSerialize::is_none")]
        });
    }
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
    let container_ident = &container.ident;
    let ser_lifetime = &context.ser_lifetime;

    let variants_ident = variants.iter().map(|v| &v.ident).collect::<Vec<_>>();
    let variants_ty = variants.iter().map(|v| &v.ty).collect::<Vec<_>>();

    let output = quote::quote! {
        const _: () = {
            #[derive(serde::Serialize)]
            #[serde(rename_all = "camelCase")]
            pub enum ToFormat<#ser_lifetime> {
                #(
                    #variants_ident(<#variants_ty as crate::deser::SubsonicSerialize<#ser_lifetime>>::Output),
                )*
            }

            impl<#ser_lifetime> crate::deser::SubsonicSerialize<#ser_lifetime> for #container_ident {
                type Input = &#ser_lifetime Self;
                type Output = ToFormat<#ser_lifetime>;

                fn prepare(input: Self::Input, format: crate::deser::Format, version: crate::common::Version) -> Self::Output {
                    match input {
                        #(
                            #container_ident::#variants_ident(input) => ToFormat::#variants_ident(<#variants_ty as crate::deser::SubsonicSerialize<#ser_lifetime>>::prepare(input, format, version)),
                        )*
                    }
                }
            }
        };
    };
    Ok(output)
}
