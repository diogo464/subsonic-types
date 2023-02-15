use std::cell::RefCell;

use proc_macro2::{Span, TokenStream};
use syn::{
    punctuated::Punctuated, spanned::Spanned, Attribute, DataStruct, DeriveInput, Meta, Path, Token,
};

#[derive(Debug, Default)]
struct Context {
    errors: RefCell<Vec<syn::Error>>,
}

impl Context {
    fn new() -> Self {
        Self::default()
    }

    fn error_at(&self, span: Span, message: &str) {
        self.errors
            .borrow_mut()
            .push(syn::Error::new(span, message));
    }

    fn error(&self, message: &str) {
        self.error_at(Span::call_site(), message);
    }

    fn into_result(self) -> Result<(), Vec<syn::Error>> {
        let errors = self.errors.take();
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

struct Container<'a> {
    attributes: Attrs,
    fields: Vec<Field<'a>>,
}

impl<'a> Container<'a> {
    fn from_input(ctx: &Context, input: &'a DeriveInput) -> Self {
        let container_attrs = Attrs::from_ast(ctx, &input.attrs);
        let fields: Vec<&syn::Field> = match &input.data {
            syn::Data::Struct(data) => data.fields.iter().collect(),
            syn::Data::Enum(_) => todo!(),
            syn::Data::Union(_) => todo!(),
        };
        Self {
            attributes: container_attrs,
            fields: Field::from_fields(ctx, &fields),
        }
    }
}

struct Field<'a> {
    attrs: Attrs,
    ty: &'a syn::Type,
    member: syn::Member,
    original: &'a syn::Field,
}

impl<'a> Field<'a> {
    fn from_ast(ctx: &Context, field: &'a syn::Field, idx: usize) -> Self {
        let attrs = Attrs::from_ast(ctx, &field.attrs);
        let member = match field.ident {
            Some(ref ident) => syn::Member::Named(ident.clone()),
            None => syn::Member::Unnamed(syn::Index::from(idx)),
        };
        Self {
            attrs,
            ty: &field.ty,
            member,
            original: field,
        }
    }

    fn from_fields(ctx: &Context, fields: &[&'a syn::Field]) -> Vec<Field<'a>> {
        fields
            .iter()
            .enumerate()
            .map(|(i, field)| Self::from_ast(ctx, field, i))
            .collect()
    }
}

const ATTR_XML: AttrName = AttrName::new("xml");
const ATTR_JSON: AttrName = AttrName::new("json");
const ATTR_COMM: AttrName = AttrName::new("common");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AttrName(&'static str);

impl AttrName {
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }
}

impl PartialEq<Path> for AttrName {
    fn eq(&self, other: &Path) -> bool {
        other.is_ident(self.0)
    }
}

// impl PartialEq<&Path> for AttrName {
//     fn eq(&self, other: &&Path) -> bool {
//          todo!()
//     }
// }

/// Containts the serde attributes that should be applied based on the format.
/// Example:
/// ```ignore
/// #[derive(SubsonicType)]
/// #[subsonic(
///     xml(rename = "@status"),
///     json(rename = "status"),
///     common()
/// )]
/// struct Response {
///    status: String,
/// }
/// ```
struct Attrs {
    xml: Option<Punctuated<syn::NestedMeta, Token![,]>>,
    json: Option<Punctuated<syn::NestedMeta, Token![,]>>,
    common: Option<Punctuated<syn::NestedMeta, Token![,]>>,
}

impl Attrs {
    fn xml_attrs(&self) -> TokenStream {
        let xml = self.xml.iter().flat_map(std::convert::identity);
        let common = self.common.iter().flat_map(std::convert::identity);
        quote::quote! {
            #[serde(#(#xml,)*#(#common,)*)]
        }
    }

    fn json_attrs(&self) -> TokenStream {
        let json = self.json.iter().flat_map(std::convert::identity);
        let common = self.common.iter().flat_map(std::convert::identity);
        quote::quote! {
            #[serde(#(#json,)*#(#common,)*)]
        }
    }

    fn from_ast(ctx: &Context, attrs: &[Attribute]) -> Self {
        let mut xml = None;
        let mut json = None;
        let mut common = None;

        for meta in Self::meta_from_attrs(ctx, attrs) {
            match meta {
                Meta::List(list) => {
                    if !list.path.is_ident("subsonic") {
                        continue;
                    }
                    for nested in list.nested {
                        let nested_span = nested.span();
                        match nested {
                            // Parse XML attributes
                            syn::NestedMeta::Meta(Meta::List(list)) if ATTR_XML == list.path => {
                                if xml.is_none() {
                                    xml = Some(list.nested);
                                } else {
                                    ctx.error_at(nested_span, "Duplicate xml attribute list");
                                }
                            }

                            // Parse Json attributes
                            syn::NestedMeta::Meta(Meta::List(list)) if ATTR_JSON == list.path => {
                                if json.is_none() {
                                    json = Some(list.nested);
                                } else {
                                    ctx.error_at(nested_span, "Duplicate json attribute list");
                                }
                            }

                            // Parse Common attributes
                            syn::NestedMeta::Meta(Meta::List(list)) if ATTR_COMM == list.path => {
                                if common.is_none() {
                                    common = Some(list.nested);
                                } else {
                                    ctx.error_at(nested_span, "Duplicate common attribute list");
                                }
                            }
                            _ => {
                                ctx.error_at(
                                    nested.span(),
                                    "Attribute must be a list, ex: \"xml(...)\"",
                                );
                                continue;
                            }
                        };
                    }
                }
                _ => ctx.error_at(meta.span(), "Invalid attribute"),
            }
        }

        Self { xml, json, common }
    }

    fn meta_from_attrs<'ctx: 'attrs, 'attrs>(
        ctx: &'ctx Context,
        attrs: &'attrs [Attribute],
    ) -> impl Iterator<Item = Meta> + 'attrs {
        attrs
            .iter()
            .map(|attr| attr.parse_meta())
            .filter_map(|meta| match meta {
                Ok(meta) => Some(meta),
                Err(err) => {
                    ctx.error_at(err.span(), &err.to_string());
                    None
                }
            })
    }
}

fn expand_subsonic_type(input: DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
    let context = Context::default();
    let container_attrs = Attrs::from_ast(&context, &input.attrs);
    let container_ident = &input.ident;

    let json_attrs = container_attrs.json_attrs();
    let xml_attrs = container_attrs.xml_attrs();

    let output = match input.data {
        syn::Data::Struct(data) => {
            let field_ident = data.fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
            let field_ty = data.fields.iter().map(|f| &f.ty).collect::<Vec<_>>();
            let field_attr = data
                .fields
                .iter()
                .map(|f| Attrs::from_ast(&context, &f.attrs))
                .collect::<Vec<_>>();
            let field_attr_json = field_attr
                .iter()
                .map(|f| f.json_attrs())
                .collect::<Vec<_>>();
            let field_attr_xml = field_attr.iter().map(|f| f.xml_attrs()).collect::<Vec<_>>();

            quote::quote! {
                impl<'de> crate::SubsonicType<'de> for #container_ident  {
                    fn deserialize<D>(format: crate::Format, deserializer: D) -> Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        match format {
                            crate::Format::Json => {
                                #[derive(serde::Deserialize)]
                                #json_attrs
                                struct Json {
                                    #(
                                        #field_attr_json
                                        #field_ident: #field_ty,
                                    )*
                                }
                                <Json as serde::Deserialize>::deserialize(deserializer).map(|json| {
                                    Self {
                                        #(#field_ident: json.#field_ident,)*
                                    }
                                })
                            }
                            crate::Format::QuickXml => {
                                #[derive(serde::Deserialize)]
                                #xml_attrs
                                struct Xml {
                                    #(
                                        #field_attr_xml
                                        #field_ident: #field_ty,
                                    )*
                                }
                                <Xml as serde::Deserialize>::deserialize(deserializer).map(|xml| {
                                    Self {
                                        #(#field_ident: xml.#field_ident,)*
                                    }
                                })
                            }
                        }
                    }

                    fn serialize<S>(&self, format: crate::Format, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: serde::Serializer,
                    {
                        match format {
                            crate::Format::Json => {
                                #[derive(serde::Serialize)]
                                #json_attrs
                                struct Json<'a> {
                                    #(
                                        #field_attr_json
                                        #field_ident: &'a #field_ty,
                                    )*
                                }
                                let json = Json {
                                    #(#field_ident: &self.#field_ident),*
                                };
                                <Json as serde::Serialize>::serialize(&json, serializer)
                            }
                            crate::Format::QuickXml => {
                                #[derive(serde::Serialize)]
                                #xml_attrs
                                struct Xml<'a> {
                                    #(
                                        #field_attr_xml
                                        #field_ident: &'a #field_ty,
                                    )*
                                }
                                let xml = Xml {
                                    #(#field_ident: &self.#field_ident),*
                                };
                                <Xml as serde::Serialize>::serialize(&xml, serializer)
                            }
                        }
                    }
                }
            }
        }
        syn::Data::Enum(data) => {
            quote::quote! {
                impl<'de> crate::SubsonicType<'de> for #container_ident  {
                    fn deserialize<D>(_format: crate::Format, deserializer: D) -> Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        <#container_ident as serde::Deserialize>::deserialize(deserializer)
                    }

                    fn serialize<S>(&self, _format: crate::Format, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: serde::Serializer,
                    {
                        <#container_ident as serde::Serialize>::serialize(self, serializer)
                    }
                }
            }
        }
        syn::Data::Union(_) => todo!(),
    };
    Ok(output)
}

fn expand_errors(errors: Vec<syn::Error>) -> TokenStream {
    let errors = errors.into_iter().map(|e| e.to_compile_error());
    quote::quote! {#(#errors)*}
}

#[proc_macro_derive(SubsonicType, attributes(subsonic))]
pub fn subsonic_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    proc_macro::TokenStream::from(expand_subsonic_type(input).unwrap_or_else(expand_errors))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let context = Context::new();
        let input: DeriveInput = syn::parse_quote! {
            #[subsonic(xml(rename="@status"), json(rename="status"))]
            struct MyStruct {}
        };

        let attrs = Attrs::from_ast(&context, &input.attrs);
        context.into_result().unwrap();

        assert!(attrs.xml.is_some());
        assert!(attrs.json.is_some());
        assert!(attrs.common.is_none());
    }
}
