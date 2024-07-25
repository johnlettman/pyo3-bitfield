extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{DeriveInput, Data, parse_macro_input, TypePath};
use syn::Type::Path;
use quote::{format_ident, quote};


/// Generate Python bindings for a bitfield struct using procedural macros.
#[proc_macro_derive(PyBitfield)]
pub fn py_bitfield(input: TokenStream) -> TokenStream {
    // parse the input tokens
    let input = parse_macro_input!(input as DeriveInput);

    // resolve the struct, ensuring this is actually a struct
    let Data::Struct(structure) = input.data else {
        unimplemented!();
    };

    // ensure the struct isn't empty
    if structure.fields.is_empty() {
        return TokenStream::new();
    }

    // grab the name of the struct
    let structure_ident = &input.ident;

    // generate Python getters and setters
    let fns = structure.fields.iter()
        .filter(|field| {
            let field_ident = field.ident.as_ref().expect("Expected named fields");
            !field_ident.to_string().starts_with("__")
        })
        .map(|field| {
            // grab metadata about this field
            let field_ident = field.ident.as_ref().expect("Expected named fields");
            let field_ty = &field.ty;
            let mut field_ty_expanded = quote! { #field_ty };

            if let Path(TypePath { path, .. }) = field_ty {
                if let Some(segment) = path.segments.first() {
                    let field_ty_ident = &segment.ident;
                    let field_ty_str = field_ty_ident.to_string();

                    if field_ty_str.starts_with('B') && field_ty_str[1..].parse::<u8>().is_ok() {
                        field_ty_expanded = quote! {
                            <#field_ty_ident as modular_bitfield::Specifier>::InOut
                        };
                    }
                }
            }

            // generate the bitfield get/set function idents
            let fn_get_ident = format_ident!("{}", field_ident);
            let fn_set_ident = format_ident!("set_{}", field_ident);

            // generate the pymethod get/set function idents
            let fn_getter_ident = format_ident!("py_get_{}", field_ident);
            let fn_setter_ident = format_ident!("py_set_{}", field_ident);

            // generate the pymethod get/set function documentation comments
            let doc_getter = format!("Get the value of `{field_ident}` using Python.");
            let doc_setter = format!("Set the value of `{field_ident}` using Python.");

            quote! {
                #[doc = #doc_getter]
                #[getter(#field_ident)]
                fn #fn_getter_ident(&self) -> #field_ty_expanded {
                    self.#fn_get_ident()
                }

                #[doc = #doc_setter]
                #[setter(#field_ident)]
                fn #fn_setter_ident(&mut self, #field_ident: #field_ty_expanded) {
                    self.#fn_set_ident(#field_ident)
                }
            }
        });

    // construct the final struct impl for pymethods
    let structure_impl = quote! {
        #[pymethods]
        impl #structure_ident {
            #(#fns)*
        }
    };

    // emit the tokens
    structure_impl.into()
}