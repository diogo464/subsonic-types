mod subsonic_type;

#[proc_macro_derive(SubsonicType, attributes(subsonic))]
pub fn subsonic(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match subsonic_type::expand(input) {
        Ok(output) => proc_macro::TokenStream::from(output),
        Err(err) => proc_macro::TokenStream::from(err.into_compile_error()),
    }
}
