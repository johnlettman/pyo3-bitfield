use proc_macro2::TokenStream;
use quote::quote;
use crate::Intermediate;

pub fn codegen(intermediate: Intermediate) -> TokenStream {
    let Intermediate { struct_ident, fields } = intermediate;
    if fields.len() == 0 {
        return TokenStream::new()
    }

    quote!{
        #[pyo3::pymethods]
        impl #struct_ident {
            #(#fields)*
        }
    }
}