use proc_macro2::TokenStream;
use syn::Result;

use crate::{
    attr::{self, AttrName},
    util,
};

pub const RENAME: AttrName = AttrName::new("rename");
pub const FLATTEN: AttrName = AttrName::new("flatten");

#[derive(Debug)]
pub struct ContainerAttr;

impl ContainerAttr {
    pub fn from_attrs(_attrs: &[syn::Attribute]) -> Result<Self> {
        Ok(Self)
    }
}

#[derive(Debug)]
pub struct FieldAttr {
    /// The name to use for the field in the query.
    pub rename: Option<String>,
    /// Should the field be flattened
    pub flatten: bool,
}

impl FieldAttr {
    pub fn from_attrs(attrs: &[syn::Attribute]) -> Result<Self> {
        let metas = attr::obtain_named_meta_list("query", attrs)?;
        let mut rename = None;
        let mut flatten = false;

        for meta in metas {
            match meta {
                syn::Meta::NameValue(nv) if RENAME == nv.path => {
                    if let syn::Lit::Str(s) = &nv.lit {
                        rename = Some(s.value());
                    }
                }
                syn::Meta::Path(p) if FLATTEN == p => {
                    flatten = true;
                }
                _ => return Err(syn::Error::new_spanned(meta, "Invalid subsonic attribute")),
            }
        }

        Ok(Self { rename, flatten })
    }
}

#[derive(Debug)]
pub struct Container<'a> {
    pub ident: &'a syn::Ident,
    pub generics: &'a syn::Generics,
    pub attrs: ContainerAttr,
    pub data: Data<'a>,
    pub input: &'a syn::DeriveInput,
}

#[derive(Debug)]
pub struct Field<'a> {
    pub ident: &'a syn::Ident,
    pub ty: &'a syn::Type,
    pub attrs: FieldAttr,
}

#[derive(Debug)]
pub enum Data<'a> {
    Struct(Vec<Field<'a>>),
}

impl<'a> Container<'a> {
    pub fn from_input(input: &'a syn::DeriveInput) -> Result<Self> {
        let ident = &input.ident;
        let generics = &input.generics;
        let attrs = ContainerAttr::from_attrs(&input.attrs)?;
        let data = Data::from_input(input)?;
        Ok(Self {
            ident,
            generics,
            attrs,
            data,
            input,
        })
    }
}

impl<'a> Data<'a> {
    fn from_input(input: &'a syn::DeriveInput) -> Result<Self> {
        match &input.data {
            syn::Data::Struct(data) => Self::from_struct_data(data),
            _ => Err(syn::Error::new_spanned(input, "Only structs are supported")),
        }
    }

    fn from_struct_data(data: &'a syn::DataStruct) -> Result<Self> {
        let mut fields = Vec::with_capacity(data.fields.len());
        for field in data.fields.iter() {
            fields.push(Field {
                ident: field.ident.as_ref().ok_or_else(|| {
                    syn::Error::new_spanned(field, "Unnamed fields are not supported")
                })?,
                ty: &field.ty,
                attrs: FieldAttr::from_attrs(&field.attrs)?,
            })
        }
        Ok(Data::Struct(fields))
    }
}

pub fn to_query(input: syn::DeriveInput) -> Result<TokenStream> {
    let container = Container::from_input(&input)?;
    match &container.data {
        Data::Struct(data) => to_query_struct(&container, data),
    }
}

pub fn to_query_struct(container: &Container, fields: &[Field]) -> Result<TokenStream> {
    let container_ident = container.ident;
    let (impl_g, type_g, where_g) = container.generics.split_for_impl();
    let fields = fields_to_query(fields);

    let output = quote::quote! {
        #[automatically_derived]
        impl #impl_g crate::query::ToQuery for #container_ident #type_g #where_g {
            fn to_query_builder<B>(&self, builder: &mut B)
            where
                B: crate::query::QueryBuilder,
            {
                #(#fields)*
            }
        }
    };
    Ok(output)
}

pub fn from_query(input: syn::DeriveInput) -> Result<TokenStream> {
    let container = Container::from_input(&input)?;
    match &container.data {
        Data::Struct(data) => from_query_struct(&container, data),
    }
}

pub fn from_query_struct(container: &Container, fields: &[Field]) -> Result<TokenStream> {
    let container_ident = container.ident;
    let (impl_g, type_g, where_g) = container.generics.split_for_impl();

    let fields_ident = fields.iter().map(|field| field.ident).collect::<Vec<_>>();
    let fields_accum_ty = fields_accum_struct_type(fields);

    let consume_arms = fields_consume_match_arm(fields);
    let flat_conume = fields_flattened_consume(fields);
    let fields_finish = fields_finish(fields);

    let output = quote::quote! {
        const _: () = {
            pub struct Accum #impl_g #where_g {
                #(
                    #fields_ident: #fields_accum_ty,
                )*
            }

            impl #impl_g Default for Accum #type_g #where_g {
                fn default() -> Self {
                    Self {
                        #(
                            #fields_ident: Default::default(),
                        )*
                    }
                }
            }

            #[automatically_derived]
            impl #impl_g crate::query::QueryAccumulator for Accum #type_g #where_g {
                type Output = #container_ident #type_g;

                fn consume<'a>(&mut self, pair: crate::query::QueryPair<'a>) -> crate::query::Result<crate::query::ConsumeStatus<'a>> {
                    use crate::query::QueryAccumulator;
                    use crate::query::QueryValueAccumulator;

                    match pair.key.as_ref() {
                        #(#consume_arms)*
                        _ => {
                            #(#flat_conume)*
                            Ok(crate::query::ConsumeStatus::Ignored(pair))
                        }
                    }
                }

                fn finish(self) -> crate::query::Result<Self::Output> {
                    use crate::query::QueryAccumulator;
                    use crate::query::QueryValueAccumulator;

                    #(#fields_finish)*

                    Ok(#container_ident {
                        #(#fields_ident,)*
                    })
                }
            }

            impl #impl_g crate::query::FromQuery for #container_ident #type_g #where_g {
                type QueryAccumulator = Accum #type_g #where_g;
            } 
        };
    };
    Ok(output)
}

fn fields_finish(fields: &[Field]) -> Vec<TokenStream> {
    fields.iter().map(field_finish).collect()
}

fn field_finish(field: &Field) -> TokenStream {
    let field_ident = field.ident;
    let field_name = field_ident.to_string();

    if field.attrs.flatten {
        quote::quote! {
            let #field_ident = self.#field_ident.finish()?;
        }
    } else {
        quote::quote! {
            let #field_ident = self
                .#field_ident
                .finish()
                .map_err(|e| crate::query::QueryParseError::invalid_value(#field_name, e))?;
        }
    }
}

fn fields_flattened_consume(fields: &[Field]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(field_flattened_consume)
        .collect()
}

fn field_flattened_consume(field: &Field) -> TokenStream {
    if !field.attrs.flatten {
        return Default::default();
    }
    let field_ident = field.ident;

    quote::quote! {
        let pair = match self.#field_ident.consume(pair)? {
            crate::query::ConsumeStatus::Consumed => return Ok(crate::query::ConsumeStatus::Consumed),
            crate::query::ConsumeStatus::Ignored(pair) => pair,
        };
    }
}

fn fields_to_query(fields: &[Field]) -> Vec<TokenStream> {
    fields.iter().map(field_to_query).collect()
}

fn field_to_query(field: &Field) -> TokenStream {
    let field_ty = field.ty;
    let field_ident = field.ident;
    let field_name = field_name(field);
    
    if field.attrs.flatten {
        quote::quote! {
            <#field_ty as crate::query::ToQuery>::to_query_builder(
                &self.#field_ident,
                builder,
            );
        }
    } else {
        quote::quote! {
            <#field_ty as crate::query::ToQueryValue>::to_query_builder(
                &self.#field_ident,
                builder,
                #field_name,
            );
        }
    }
}

fn fields_consume_match_arm(fields: &[Field]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(field_consume_match_arm)
        .collect()
}

fn field_consume_match_arm(field: &Field) -> TokenStream {
    if field.attrs.flatten {
        return Default::default();
    }

    let field_ident = field.ident;
    let field_name = field_name(field);

    quote::quote! {
        #field_name => {
            self.#field_ident
                .consume(pair.value)
                .map_err(|e| crate::query::QueryParseError::invalid_value(#field_name, e))?;
            Ok(crate::query::ConsumeStatus::Consumed)
        }
    }
}

fn fields_accum_struct_type(fields: &[Field]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(field_accum_struct_type)
        .collect()
}

fn field_accum_struct_type(field: &Field) -> TokenStream {
    let field_ty = field.ty;
    if field.attrs.flatten {
        quote::quote! {
            <#field_ty as crate::query::FromQuery>::QueryAccumulator
        }
    } else {
        quote::quote! {
            <#field_ty as crate::query::FromQueryValue>::QueryValueAccumulator
        }
    }
}

// Common

fn field_name(field: &Field) -> String {
    match field.attrs.rename {
        Some(ref name) => name.clone(),
        None => util::string_to_camel_case(&field.ident.to_string()),
    }
}
