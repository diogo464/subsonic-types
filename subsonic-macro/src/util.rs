pub fn string_to_camel_case(string: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    for c in string.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}

pub fn type_is_vec(ty: &syn::Type) -> bool {
    if let syn::Type::Path(syn::TypePath { path, .. }) = ty {
        if let Some(syn::PathSegment { ident, .. }) = path.segments.last() {
            ident == "Vec"
        } else {
            false
        }
    } else {
        false
    }
}

pub fn type_is_option(ty: &syn::Type) -> bool {
    if let syn::Type::Path(syn::TypePath { path, .. }) = ty {
        if let Some(syn::PathSegment { ident, .. }) = path.segments.last() {
            ident == "Option"
        } else {
            false
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_case() {
        assert_eq!(string_to_camel_case("foo_bar"), "fooBar");
        assert_eq!(string_to_camel_case("foo_bar_baz"), "fooBarBaz");
        assert_eq!(string_to_camel_case("foo_bar_baz_qux"), "fooBarBazQux");

        assert_eq!(string_to_camel_case("foo"), "foo");
        assert_eq!(string_to_camel_case("foo_bar"), "fooBar");
    }

    #[test]
    fn test_is_vec() {
        assert!(type_is_vec(&syn::parse_quote! { Vec<foo::Bar> }));
        assert!(!type_is_vec(&syn::parse_quote! { foo::Bar }));
    }

    #[test]
    fn test_is_option() {
        assert!(type_is_option(&syn::parse_quote! { Option<foo::Bar> }));
        assert!(!type_is_option(&syn::parse_quote! { foo::Bar }));
    }
}
