use syn::Result;

use crate::{attr, common::Version, util};

struct ContainerAttributes {
    since: Version,
    path: String,
}

impl ContainerAttributes {
    fn extract(attrs: &mut Vec<syn::Attribute>) -> Result<Self> {
        let metas = attr::extract_meta_list(attrs)?;
        let mut since = None;
        let mut path = None;

        for meta in metas {
            match &meta {
                syn::Meta::NameValue(
                    meta @ syn::MetaNameValue {
                        lit: syn::Lit::Str(value),
                        ..
                    },
                ) if attr::SINCE == meta.path => {
                    if since.is_some() {
                        return Err(syn::Error::new_spanned(meta, "Duplicate attribute"));
                    } else {
                        let v = Version::parse(&value.value())
                            .ok_or_else(|| syn::Error::new_spanned(meta, "Invalid version"))?;
                        since = Some(v);
                    }
                }
                syn::Meta::NameValue(
                    meta @ syn::MetaNameValue {
                        lit: syn::Lit::Str(value),
                        ..
                    },
                ) if attr::PATH == meta.path => {
                    if path.is_some() {
                        return Err(syn::Error::new_spanned(meta, "Duplicate attribute"));
                    } else {
                        path = Some(value.value());
                    }
                }
                _ => return Err(syn::Error::new_spanned(meta, "Invalid subsonic attribute")),
            }
        }

        let since = since.ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "Missing since attribute on subsonic attribute",
            )
        })?;
        let path = path.ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "Missing path attribute on subsonic attribute",
            )
        })?;

        Ok(Self { since, path })
    }
}

struct FieldAttributes {
    flatten: bool,
}

impl FieldAttributes {
    fn obtain(attrs: &Vec<syn::Attribute>) -> Result<Self> {
        let metas = attr::obtain_meta_list(attrs)?;
        let mut flatten = false;

        for meta in metas {
            match meta {
                syn::Meta::Path(path) if attr::FLATTEN == path => {
                    if flatten {
                        return Err(syn::Error::new_spanned(path, "Duplicate flatten attribute"));
                    } else {
                        flatten = true;
                    }
                }
                _ => return Err(syn::Error::new_spanned(meta, "Invalid subsonic attribute")),
            }
        }

        Ok(Self { flatten })
    }
}

fn input_get_data_struct(input: &syn::DeriveInput) -> Result<&syn::DataStruct> {
    match &input.data {
        syn::Data::Struct(data) => Ok(data),
        _ => Err(syn::Error::new_spanned(
            input,
            "Only structs can be used with #[derive(SubsonicRequest)]",
        )),
    }
}

fn field_get_name_str(field: &syn::Field) -> Result<String> {
    field
        .ident
        .as_ref()
        .map(|i| i.to_string())
        .as_deref()
        .map(util::string_to_camel_case)
        .ok_or_else(|| syn::Error::new_spanned(field, "Field must be named"))
}

fn expand_to_query(input: syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
    let data = input_get_data_struct(&input)?;
    let ident = &input.ident;

    // The statements to build each field in the query
    // Example for a regular field:
    // <u32 as crate::query::ToQueryValue>::to_query_builder(&self.id, builder, "id");
    // Example for a flattened field:
    // <Foo as crate::query::ToQuery>::to_query_builder(&self.foo, builder);
    let mut query_build_stmts = Vec::new();
    for field in data.fields.iter() {
        let field_ty = &field.ty;
        let field_ident = &field.ident;
        let field_ident_str = field_get_name_str(field)?;

        let attrs = FieldAttributes::obtain(&field.attrs)?;
        let stmt = if attrs.flatten {
            quote::quote! {
                <#field_ty as crate::query::ToQuery>::to_query_builder(&self.#field_ident, builder);
            }
        } else {
            quote::quote! {
                <#field_ty as crate::query::ToQueryValue>::to_query_builder(&self.#field_ident, builder, #field_ident_str);
            }
        };
        query_build_stmts.push(stmt);
    }

    let output = quote::quote! {
        #[automatically_derived]
        impl crate::query::ToQuery for #ident {
            fn to_query_builder<B>(&self, builder: &mut B)
            where
                B: crate::query::QueryBuilder,
            {
                #(#query_build_stmts)*
            }
        }
    };
    Ok(output)
}

fn expand_from_query(input: syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
    fn expand_accum_struct(
        ident: &syn::Ident,
        data: &syn::DataStruct,
    ) -> Result<proc_macro2::TokenStream> {
        let mut fields = Vec::new();
        for field in data.fields.iter() {
            let attrs = FieldAttributes::obtain(&field.attrs)?;
            let field_ty = &field.ty;
            let field_ident = &field.ident;
            if attrs.flatten {
                fields.push(quote::quote! {
                    #field_ident: <#field as crate::query::FromQuery>::QueryAccumulator
                });
            } else {
                fields.push(quote::quote! {
                    #field_ident: <#field_ty as crate::query::FromQueryValue>::QueryValueAccumulator
                });
            }
        }

        let output = quote::quote! {
            #[derive(Default)]
            pub struct #ident {
                #(#fields,)*
            }
        };
        Ok(output)
    }

    fn expand_accum_impl(
        ident: &syn::Ident,
        input: &syn::DeriveInput,
    ) -> Result<proc_macro2::TokenStream> {
        let input_ident = &input.ident;
        let data = input_get_data_struct(&input)?;

        let field_ident = data.fields.iter().map(|f| &f.ident);

        let mut regular_field_match_args = Vec::new();
        let mut regular_field_finish = Vec::new();

        let mut flattened_field_match_args = Vec::new();

        for field in data.fields.iter() {
            let attrs = FieldAttributes::obtain(&field.attrs)?;
            let field_ident = &field.ident;
            let field_ident_str = field_get_name_str(field)?;

            if attrs.flatten {
                let arm = quote::quote! {};
                flattened_field_match_args.push(arm);
            } else {
                let arm = quote::quote! {
                    #field_ident_str => {
                        self.#field_ident
                            .consume(pair.value)
                            .map_err(|e| crate::query::QueryParseError::invalid_value(#field_ident_str, e))?;
                        Ok(crate::query::ConsumeStatus::Consumed)
                    }
                };
                let finish = quote::quote! {
                    let #field_ident = self.#field_ident
                        .finish()
                        .map_err(|e| crate::query::QueryParseError::invalid_value(#field_ident_str, e))?;
                };
                regular_field_match_args.push(arm);
                regular_field_finish.push(finish);
            }
        }

        let output = quote::quote! {
            #[automatically_derived]
            impl crate::query::QueryAccumulator for #ident {
                type Output = #input_ident;

                fn consume<'a>(&mut self, pair: crate::query::QueryPair<'a>) -> crate::query::Result<crate::query::ConsumeStatus<'a>> {
                    use crate::query::QueryAccumulator;
                    use crate::query::QueryValueAccumulator;

                    match pair.key.as_ref() {
                        #(#regular_field_match_args)*
                        _ => Ok(crate::query::ConsumeStatus::Ignored(pair)),
                    }
                }

                fn finish(self) -> crate::query::Result<Self::Output> {
                    use crate::query::QueryAccumulator;
                    use crate::query::QueryValueAccumulator;

                    #(#regular_field_finish)*

                    Ok(#input_ident { #(#field_ident),*})
                }
            }
        };
        Ok(output)
    }

    let data = input_get_data_struct(&input)?;
    let ident = &input.ident;
    let accum_ident = quote::format_ident!("{}Accum", ident);

    let accum_struct = expand_accum_struct(&accum_ident, &data)?;
    let accum_impl = expand_accum_impl(&accum_ident, &input)?;
    let output = quote::quote! {
        const _: () = {
            #accum_struct
            #accum_impl

            #[automatically_derived]
            impl crate::query::FromQuery for #ident {
                type QueryAccumulator = #accum_ident;
            }
        };
    };
    Ok(output)
}

pub fn expand(mut input: syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
    let impl_to_query = expand_to_query(input.clone())?;
    let impl_from_query = expand_from_query(input.clone())?;

    let container_attrs = ContainerAttributes::extract(&mut input.attrs)?;
    let container_ident = &input.ident;

    let path = &container_attrs.path;
    let since = &container_attrs.since;
    let output = quote::quote! {
        impl crate::request::SubsonicRequest for #container_ident {
            const PATH: &'static str = #path;
            const SINCE: crate::common::Version = #since;
        }

        #impl_to_query
        #impl_from_query
    };

    Ok(output)
}
