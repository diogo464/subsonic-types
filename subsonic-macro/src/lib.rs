mod attr;
mod request;
mod response;
mod util;
mod version;

#[proc_macro_derive(SubsonicType, attributes(subsonic))]
pub fn subsonic(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match response::expand(input) {
        Ok(output) => proc_macro::TokenStream::from(output),
        Err(err) => proc_macro::TokenStream::from(err.into_compile_error()),
    }
}

#[proc_macro_derive(SubsonicRequest, attributes(subsonic))]
pub fn subsonic_request(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match request::expand(input) {
        Ok(output) => proc_macro::TokenStream::from(output),
        Err(err) => proc_macro::TokenStream::from(err.into_compile_error()),
    }
}
