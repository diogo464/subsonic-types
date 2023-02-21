use syn::Result;

pub const RENAME: AttrName = AttrName::new("rename");
pub const FLATTEN: AttrName = AttrName::new("flatten");
pub const SINCE: AttrName = AttrName::new("since");
pub const PATH: AttrName = AttrName::new("path");
pub const ATTRIBUTE: AttrName = AttrName::new("attribute");
pub const OPTIONAL: AttrName = AttrName::new("optional");
pub const CHOICE: AttrName = AttrName::new("choice");
pub const VALUE: AttrName = AttrName::new("value");

pub struct AttrName(&'static str);

impl AttrName {
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }
}

impl std::cmp::PartialEq<syn::Path> for AttrName {
    fn eq(&self, other: &syn::Path) -> bool {
        other.is_ident(self.0)
    }
}

impl std::cmp::PartialEq<syn::Ident> for AttrName {
    fn eq(&self, other: &syn::Ident) -> bool {
        other == self.0
    }
}

impl std::cmp::PartialEq<&syn::Path> for AttrName {
    fn eq(&self, other: &&syn::Path) -> bool {
        other.is_ident(self.0)
    }
}

impl std::cmp::PartialEq<&syn::Ident> for AttrName {
    fn eq(&self, other: &&syn::Ident) -> bool {
        other == &self.0
    }
}

pub fn vec_to_meta(vec: Vec<syn::Attribute>) -> Result<Vec<syn::Meta>> {
    let mut metas = Vec::new();
    for attr in vec {
        metas.push(attr.parse_meta()?);
    }
    Ok(metas)
}

pub fn slice_to_meta(slice: &[syn::Attribute]) -> Result<Vec<syn::Meta>> {
    let mut metas = Vec::new();
    for attr in slice {
        metas.push(attr.parse_meta()?);
    }
    Ok(metas)
}

pub fn obtain_meta_list(attrs: &[syn::Attribute]) -> Result<Vec<syn::Meta>> {
    extract_named_meta_list("subsonic", &mut attrs.to_vec())
}

/// Extract subsonic attribute lists and return all nested Meta
pub fn extract_meta_list(attrs: &mut Vec<syn::Attribute>) -> Result<Vec<syn::Meta>> {
    extract_named_meta_list("subsonic", attrs)
}

/// Extract named attribute lists and return all nested Meta
pub fn extract_named_meta_list(
    name: &str,
    attrs: &mut Vec<syn::Attribute>,
) -> Result<Vec<syn::Meta>> {
    let mut metas = extract_named_meta(name, attrs)?;
    let mut nested = Vec::new();
    for meta in metas.drain(..) {
        match meta {
            syn::Meta::List(list) => nested.extend(list.nested),
            _ => return Err(syn::Error::new_spanned(meta, "Expected list")),
        }
    }

    metas.clear();
    for n in nested {
        match n {
            syn::NestedMeta::Meta(meta) => metas.push(meta),
            _ => return Err(syn::Error::new_spanned(n, "Invalid attribute")),
        }
    }
    Ok(metas)
}

/// Extract named attributes
pub fn extract_named(name: &str, attrs: &mut Vec<syn::Attribute>) -> Vec<syn::Attribute> {
    let mut extracted = Vec::new();
    let mut index = 0;
    while index < attrs.len() {
        let attr = &attrs[index];
        if attr.path.is_ident(name) {
            extracted.push(attr.clone());
            attrs.remove(index);
        } else {
            index += 1;
        }
    }
    extracted
}

/// Extract named attributes and convert them to Meta
pub fn extract_named_meta(name: &str, attrs: &mut Vec<syn::Attribute>) -> Result<Vec<syn::Meta>> {
    let extracted = extract_named(name, attrs);
    vec_to_meta(extracted)
}
