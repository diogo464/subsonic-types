mod subsonic_type;

type Result<T, E = syn::Error> = std::result::Result<T, E>;

const ATTR_XML: AttrName = AttrName::new("xml");
const ATTR_JSON: AttrName = AttrName::new("json");
const ATTR_COMM: AttrName = AttrName::new("common");

const SERIALIZE_STRUCT_LIFETIME: &str = "__subsonic_ser";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Format {
    Json,
    Xml,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Action {
    Serialize,
    Deserialize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AttrName(&'static str);

impl AttrName {
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }
}

impl PartialEq<syn::Path> for AttrName {
    fn eq(&self, other: &syn::Path) -> bool {
        other.is_ident(self.0)
    }
}

#[derive(Debug)]
struct SubsonicAttrs {
    json: Vec<syn::NestedMeta>,
    xml: Vec<syn::NestedMeta>,
    common: Vec<syn::NestedMeta>,
}

impl SubsonicAttrs {
    fn parse_meta(list: syn::MetaList) -> Result<Self> {
        debug_assert!(list.path.is_ident("subsonic"));
        let mut json: Option<Vec<syn::NestedMeta>> = None;
        let mut xml: Option<Vec<syn::NestedMeta>> = None;
        let mut common: Option<Vec<syn::NestedMeta>> = None;

        for nested in list.nested {
            match nested {
                syn::NestedMeta::Meta(syn::Meta::List(l)) if ATTR_JSON == l.path => {
                    if json.is_none() {
                        json = Some(l.nested.iter().cloned().collect());
                    } else {
                        return Err(syn::Error::new_spanned(l, "Duplicate json attribute list"));
                    }
                }
                syn::NestedMeta::Meta(syn::Meta::List(l)) if ATTR_XML == l.path => {
                    if xml.is_none() {
                        xml = Some(l.nested.iter().cloned().collect());
                    } else {
                        return Err(syn::Error::new_spanned(l, "Duplicate xml attribute list"));
                    }
                }
                syn::NestedMeta::Meta(syn::Meta::List(l)) if ATTR_COMM == l.path => {
                    if common.is_none() {
                        common = Some(l.nested.iter().cloned().collect());
                    } else {
                        return Err(syn::Error::new_spanned(
                            l,
                            "Duplicate common attribute list",
                        ));
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        nested,
                        "Subsonic xml, json and common attribute fields must be a list",
                    ))
                }
            };
        }

        let json = json.unwrap_or_default();
        let xml = xml.unwrap_or_default();
        let common = common.unwrap_or_default();
        Ok(Self { json, xml, common })
    }

    fn into_json(self) -> proc_macro2::TokenStream {
        Self::meta_to_token_stream(self.common.into_iter().chain(self.json.into_iter()))
    }

    fn into_xml(self) -> proc_macro2::TokenStream {
        Self::meta_to_token_stream(self.common.into_iter().chain(self.xml.into_iter()))
    }

    fn meta_to_token_stream(
        meta: impl Iterator<Item = syn::NestedMeta>,
    ) -> proc_macro2::TokenStream {
        quote::quote! {#[serde(#(#meta),*)]}
    }
}

struct InnerAttr(Option<syn::Attribute>);

impl syn::parse::Parse for InnerAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        if attrs.len() > 1 {
            return Err(syn::Error::new_spanned(
                &attrs[0],
                "Expected exactly 1 attribute in the token stream",
            ));
        }
        Ok(Self(attrs.into_iter().next()))
    }
}

fn create_lifetime_param() -> syn::GenericParam {
    let tick = proc_macro2::Punct::new('\'', proc_macro2::Spacing::Joint);
    let name = proc_macro2::Ident::new(SERIALIZE_STRUCT_LIFETIME, proc_macro2::Span::call_site());
    syn::parse_quote! {#tick #name}
}

fn parse_attribute(tokens: proc_macro2::TokenStream) -> Result<Option<syn::Attribute>> {
    let inner: InnerAttr = syn::parse2(tokens)?;
    Ok(inner.0)
}

fn append_derive(action: Action, attrs: &mut Vec<syn::Attribute>) -> Result<()> {
    // Insert at index 0 so that the `#[derive(serde::Serialize/Deserialize)]`
    // comes before the attribute `#[serde(...)]`. This avoid warning that may become an error.
    // https://github.com/rust-lang/rust/issues/79202
    match action {
        Action::Serialize => attrs.insert(0, syn::parse_quote!(#[derive(serde::Serialize)])),
        Action::Deserialize => attrs.insert(0, syn::parse_quote!(#[derive(serde::Deserialize)])),
    };
    Ok(())
}

fn generics_with_de_lifetime(generics: &syn::Generics) -> syn::Generics {
    let mut cloned = generics.clone();
    cloned.params.insert(0, create_lifetime_param());
    cloned
}

fn generics_without_bounds(generics: &syn::Generics) -> proc_macro2::TokenStream {
    let mut param_names: Vec<proc_macro2::TokenStream> = Vec::new();
    for param in generics.params.iter() {
        let name = match param {
            syn::GenericParam::Type(t) => {
                let ident = &t.ident;
                quote::quote! {#ident}
            }
            syn::GenericParam::Lifetime(lt) => quote::quote! {#lt},
            syn::GenericParam::Const(c) => {
                let ident = &c.ident;
                quote::quote! {#ident}
            }
        };
        param_names.push(name);
    }
    quote::quote! {<#(#param_names),*>}
}

fn lower_subsonic_attr(
    format: Format,
    action: Action,
    attrs: &mut Vec<syn::Attribute>,
) -> Result<bool> {
    let mut is_field = false;
    let mut index = 0;
    while index < attrs.len() {
        let attr = &mut attrs[index];
        let meta = attr.parse_meta()?;

        if meta.path().is_ident("subsonic_field") {
            is_field = true;
            attrs.remove(index);
            continue;
        }

        if !meta.path().is_ident("subsonic") {
            index += 1;
            continue;
        }

        let list = match meta {
            syn::Meta::List(l) => l,
            _ => {
                return Err(syn::Error::new_spanned(
                    attr,
                    "Subsonic attribute must be a list, ex: \"#[subsonic(...)]\"",
                ))
            }
        };
        let subsonic_attrs = SubsonicAttrs::parse_meta(list)?;
        let attr_token_stream = match format {
            Format::Json => subsonic_attrs.into_json(),
            Format::Xml => subsonic_attrs.into_xml(),
        };
        let lowered_attr = parse_attribute(attr_token_stream)?;
        *attr = lowered_attr.unwrap_or_else(|| syn::parse_quote! {#[serde()]});
        index += 1;
    }
    Ok(is_field)
}

fn is_field_subsonic(field: &syn::Field) -> bool {
    for attr in field.attrs.iter() {
        if attr.path.is_ident("subsonic_field") {
            return true;
        }
    }
    false
}

fn patch_field(format: Format, action: Action, field: &mut syn::Field) -> Result<()> {
    let is_field = lower_subsonic_attr(format, action, &mut field.attrs)?;
    // Add a reference to the fields when serializing
    // This way we dont need to clone them
    // Example:
    // ```ignore
    // struct Foo {
    //     field: String
    // }
    //
    // struct JsonSeFoo {
    //     field: &String
    // }
    // ```
    if action == Action::Serialize {
        let lifetime = create_lifetime_param();
        let field_ty = &field.ty;
        field.ty = syn::parse_quote!(&#lifetime #field_ty);
    }
    // Wrap the type in Json<> or QuickXml<>
    if is_field {
        match format {
            Format::Json => {
                let field_ty = &field.ty;
                field.ty = syn::parse_quote!(crate::Json<#field_ty>);
            }
            Format::Xml => {
                let field_ty = &field.ty;
                field.ty = syn::parse_quote!(crate::QuickXml<#field_ty>);
            }
        }
    }
    Ok(())
}

fn patch_struct_fields(format: Format, action: Action, fields: &mut syn::Fields) -> Result<()> {
    match fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter_mut()
            .try_for_each(|f| patch_field(format, action, f))?,
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter_mut()
            .try_for_each(|f| patch_field(format, action, f))?,
        syn::Fields::Unit => {}
    }
    Ok(())
}

fn patch_enum_variants<'a>(
    format: Format,
    action: Action,
    variants: impl Iterator<Item = &'a mut syn::Variant>,
) -> Result<()> {
    for variant in variants {
        lower_subsonic_attr(format, action, &mut variant.attrs)?;
        match &mut variant.fields {
            syn::Fields::Named(fields) => {
                for field in fields.named.iter_mut() {
                    patch_field(format, action, field)?;
                }
            }
            syn::Fields::Unnamed(fields) => {
                for field in fields.unnamed.iter_mut() {
                    patch_field(format, action, field)?;
                }
            }
            syn::Fields::Unit => {}
        }
    }
    Ok(())
}

fn patch_container_fields(format: Format, action: Action, data: &mut syn::Data) -> Result<()> {
    match data {
        syn::Data::Struct(data) => {
            patch_struct_fields(format, action, &mut data.fields)?;
        }
        syn::Data::Enum(data) => {
            patch_enum_variants(format, action, data.variants.iter_mut())?;
        }
        syn::Data::Union(_) => unimplemented!(),
    }
    Ok(())
}

fn patch_container_attrs(
    format: Format,
    action: Action,
    attrs: &mut Vec<syn::Attribute>,
) -> Result<()> {
    append_derive(action, attrs)?;
    lower_subsonic_attr(format, action, attrs)?;
    Ok(())
}

fn patch_container_ident(format: Format, action: Action, ident: &mut syn::Ident) -> Result<()> {
    let format = match format {
        Format::Json => "Json",
        Format::Xml => "Xml",
    };
    let action = match action {
        Action::Serialize => "Se",
        Action::Deserialize => "De",
    };
    *ident = quote::format_ident!("{}{}{}", format, action, ident);
    Ok(())
}

fn patch_container_generics(
    format: Format,
    action: Action,
    generics: &mut syn::Generics,
) -> Result<()> {
    if action == Action::Serialize {
        let lifetime = create_lifetime_param();
        generics.params.insert(0, lifetime);
    }
    Ok(())
}

fn patch_input(format: Format, action: Action, input: &mut syn::DeriveInput) -> Result<()> {
    patch_container_attrs(format, action, &mut input.attrs)?;
    patch_container_ident(format, action, &mut input.ident)?;
    patch_container_fields(format, action, &mut input.data)?;
    patch_container_generics(format, action, &mut input.generics)?;
    Ok(())
}

fn clone_and_patch_input(
    format: Format,
    action: Action,
    input: &syn::DeriveInput,
) -> Result<syn::DeriveInput> {
    let mut patched = input.clone();
    patch_input(format, action, &mut patched)?;
    Ok(patched)
}

fn expand_se_method_body(
    format: Format,
    input: &syn::DeriveInput,
    patched: &syn::DeriveInput,
) -> proc_macro2::TokenStream {
    let wrapper = match format {
        Format::Json => quote::quote! {crate::Json},
        Format::Xml => quote::quote! {crate::QuickXml},
    };
    let input_ident = &input.ident;
    let patched_ident = &patched.ident;

    match (&input.data, &patched.data) {
        (syn::Data::Struct(input_s), syn::Data::Struct(_)) => {
            let field_ident = input_s.fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
            let field_wrapper = input_s
                .fields
                .iter()
                .map(|f| {
                    if is_field_subsonic(f) {
                        wrapper.clone()
                    } else {
                        quote::quote! {}
                    }
                })
                .collect::<Vec<_>>();

            quote::quote! {
                let patched = #patched_ident {
                    #(#field_ident: #field_wrapper(&self.#field_ident),)*
                };
                <#patched_ident as serde::Serialize>::serialize(&patched, serializer)
            }
        }
        (syn::Data::Enum(input_s), syn::Data::Enum(_)) => {
            let variants = input_s
                .variants
                .iter()
                .map(|v| &v.ident)
                .collect::<Vec<_>>();
            quote::quote! {
                let patched = match &self {
                    #(#input_ident::#variants(v) => #patched_ident::#variants(#wrapper(v)),)*
                };
                <#patched_ident as serde::Serialize>::serialize(&patched, serializer)
            }
        }
        (syn::Data::Union(_), syn::Data::Union(_)) => {
            quote::quote! {}
        }
        _ => panic!("Invalid patched container"),
    }
}

fn expand_se_impl(
    input: &syn::DeriveInput,
    json_se: &syn::DeriveInput,
    xml_se: &syn::DeriveInput,
) -> Result<proc_macro2::TokenStream> {
    let input_ident = &input.ident;
    let se_generics = &input.generics;
    let se_generics_no_bounds = generics_without_bounds(&se_generics);

    let json_se_body = expand_se_method_body(Format::Json, input, json_se);
    let xml_se_body = expand_se_method_body(Format::Xml, input, xml_se);

    let output = quote::quote! {
        impl #se_generics crate::SubsonicSerialize for #input_ident #se_generics_no_bounds {
            fn serialize<S>(&self, format: crate::Format, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer
            {
                match format {
                    crate::Format::Json => {
                        #json_se_body
                    }
                    crate::Format::QuickXml => {
                        #xml_se_body
                    }
                }
            }
        }
    };

    Ok(output)
}

fn expand_de_method_body(
    input: &syn::DeriveInput,
    patched: &syn::DeriveInput,
) -> proc_macro2::TokenStream {
    let input_ident = &input.ident;
    let patched_ident = &patched.ident;

    match (&input.data, &patched.data) {
        (syn::Data::Struct(input_s), syn::Data::Struct(_)) => {
            let field_ident = input_s.fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
            let field_access = input_s
                .fields
                .iter()
                .map(|f| {
                    if is_field_subsonic(f) {
                        quote::quote! {.into_inner()}
                    } else {
                        quote::quote! {}
                    }
                })
                .collect::<Vec<_>>();

            quote::quote! {
                let patched = <#patched_ident as serde::Deserialize>::deserialize(deserialize)?;
                Ok(#input_ident {
                    #(#field_ident: patched.#field_ident #field_access,)*
                })
            }
        }
        (syn::Data::Enum(input_s), syn::Data::Enum(_)) => {
            let variants = input_s
                .variants
                .iter()
                .map(|v| &v.ident)
                .collect::<Vec<_>>();
            quote::quote! {
                let patched = <#patched_ident as serde::Deserialize>::deserialize(deserialize)?;
                Ok(match patched {
                    #(#patched_ident::#variants(v) => #input_ident::#variants(v.into_inner()),)*
                })
            }
        }
        (syn::Data::Union(_), syn::Data::Union(_)) => {
            quote::quote! {}
        }
        _ => panic!("Invalid patched container"),
    }
}

fn expand_de_impl(
    input: &syn::DeriveInput,
    json_de: &syn::DeriveInput,
    xml_de: &syn::DeriveInput,
) -> Result<proc_macro2::TokenStream> {
    let de_lifetime = create_lifetime_param();
    let input_ident = &input.ident;
    let input_generics = &input.generics;
    let input_generics_no_bounds = generics_without_bounds(&input_generics);
    let de_generics = generics_with_de_lifetime(&input.generics);

    let json_de_body = expand_de_method_body(input, json_de);
    let xml_de_body = expand_de_method_body(input, xml_de);

    let output = quote::quote! {
        impl #de_generics crate::SubsonicDeserialize< #de_lifetime > for #input_ident #input_generics_no_bounds {
            fn deserialize<D>(format: crate::Format, deserialize: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer< #de_lifetime >
            {
                match format {
                    crate::Format::Json => {
                        #json_de_body
                    }
                    crate::Format::QuickXml => {
                        #xml_de_body
                    }
                }
            }
        }
    };

    Ok(output)
}

fn expand_output(input: syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
    let json_se = clone_and_patch_input(Format::Json, Action::Serialize, &input)?;
    let json_de = clone_and_patch_input(Format::Json, Action::Deserialize, &input)?;
    let xml_se = clone_and_patch_input(Format::Xml, Action::Serialize, &input)?;
    let xml_de = clone_and_patch_input(Format::Xml, Action::Deserialize, &input)?;
    let se_impl = expand_se_impl(&input, &json_se, &xml_se)?;
    let de_impl = expand_de_impl(&input, &json_de, &xml_de)?;

    let output = quote::quote! {
        const _: () = {
            #json_se

            #json_de

            #xml_se

            #xml_de

            #se_impl

            #de_impl
        };
    };
    Ok(output)
}

#[proc_macro_derive(SubsonicType, attributes(subsonic, subsonic_field))]
pub fn subsonic_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match expand_output(input) {
        Ok(output) => proc_macro::TokenStream::from(output),
        Err(err) => proc_macro::TokenStream::from(err.into_compile_error()),
    }
}

#[proc_macro_derive(SubsonicType2, attributes(subsonic))]
pub fn subsonic(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match subsonic_type::expand(input) {
        Ok(output) => proc_macro::TokenStream::from(output),
        Err(err) => proc_macro::TokenStream::from(err.into_compile_error()),
    }
}
