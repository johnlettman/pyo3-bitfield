use proc_macro2::{TokenStream, Ident};
use proc_macro_error::abort;
use syn::{Data, DataStruct, DeriveInput};

pub struct Parsed {
    pub ident: Ident,
    pub data_struct: DataStruct,
}

const ERROR_NOT_STRUCT: &str = "item is not a struct";
const HELP_NOT_STRUCT: &str = "`#[derive(PyBitfield)]` only supports structs";

pub fn parse(item: TokenStream) -> Parsed {
    let DeriveInput { ident, data, .. } = syn::parse2(item).unwrap_or_else(unreachable);
    let Data::Struct(data_struct) = data else { abort!(ident, ERROR_NOT_STRUCT; help = HELP_NOT_STRUCT) };

    Parsed{ident, data_struct}
}

#[inline]
pub fn unreachable<T, U>(_: T) -> U {
    unreachable!("uh oh, executed unreachable execution path")
}