mod codegen;
mod analyze;
mod parse;
mod intermediate;

use parse::{parse, Parsed};
use analyze::{analyze, Model, FieldModel};
use intermediate::{generate, Intermediate};
use codegen::codegen;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro_derive(PyBitfield)]
pub fn derive_py_bitfield(input: TokenStream) -> TokenStream {
    py_bitfield(input.into()).into()
}

pub(crate) fn py_bitfield(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let parsed = parse(input);
    let analyzed = analyze(parsed);
    let generated = generate(analyzed);

    codegen(generated)
}