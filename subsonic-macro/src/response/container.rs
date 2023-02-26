use syn::Result;

use super::attr;

pub struct Container<'a> {
    pub ident: &'a syn::Ident,
    pub generics: &'a syn::Generics,
    pub attrs: attr::ContainerAttr,
    pub data: Data<'a>,
    pub input: &'a syn::DeriveInput,
}

pub enum Data<'a> {
    Struct(Vec<Field<'a>>),
    Enum(Vec<Variant<'a>>),
}

pub struct Field<'a> {
    pub ident: &'a syn::Ident,
    pub ty: &'a syn::Type,
    pub attrs: attr::FieldAttr,
}

pub struct Variant<'a> {
    pub ident: &'a syn::Ident,
    pub ty: &'a syn::Type,
}

impl<'a> Container<'a> {
    pub fn from_input(input: &'a syn::DeriveInput) -> Result<Self> {
        let ident = &input.ident;
        let generics = &input.generics;
        let attrs = attr::ContainerAttr::from_attrs(&input.attrs)?;
        let data = input_get_data(input)?;
        Ok(Self {
            ident,
            generics,
            attrs,
            data,
            input,
        })
    }
}

fn input_get_data(input: &syn::DeriveInput) -> Result<Data> {
    match &input.data {
        syn::Data::Struct(data) => data_struct_to_data(data),
        syn::Data::Enum(data) => data_enum_to_data(data),
        _ => Err(syn::Error::new_spanned(
            input,
            "Only structs and enums are supported",
        )),
    }
}

fn data_struct_to_data(data: &syn::DataStruct) -> Result<Data> {
    let mut fields = Vec::with_capacity(data.fields.len());
    for field in data.fields.iter() {
        fields.push(Field {
            ident: field.ident.as_ref().ok_or_else(|| {
                syn::Error::new_spanned(field, "Unnamed fields are not supported")
            })?,
            ty: &field.ty,
            attrs: attr::FieldAttr::from_attrs(&field.attrs)?,
        })
    }
    Ok(Data::Struct(fields))
}

/// Zero-field variants are ignored.
fn data_enum_to_data(data: &syn::DataEnum) -> Result<Data> {
    let mut variants = Vec::with_capacity(data.variants.len());

    for variant in data.variants.iter() {
        if variant.fields.len() > 1 {
            return Err(syn::Error::new_spanned(
                variant,
                "Only single-field or zero-field variants are supported",
            ));
        }
        if variant.fields.is_empty() {
            continue;
        }

        let ty = match variant.fields.iter().next() {
            Some(field) => &field.ty,
            None => unreachable!(),
        };

        variants.push(Variant {
            ident: &variant.ident,
            ty,
        })
    }

    Ok(Data::Enum(variants))
}
