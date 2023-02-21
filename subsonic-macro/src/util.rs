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
}
