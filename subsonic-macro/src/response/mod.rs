mod attr;
mod container;
mod deserialize;
mod serialize;

pub fn expand(input: syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let serialize_tokens = serialize::expand(&input)?;
    let deserialize_tokens = deserialize::expand(&input)?;

    let output = quote::quote! {
        #serialize_tokens
        #deserialize_tokens
    };

    Ok(output)
}
