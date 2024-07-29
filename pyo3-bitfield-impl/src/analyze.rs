use proc_macro2::{Ident, TokenStream};
use proc_macro_error::abort_call_site;
use quote::{quote, ToTokens};
use regex::Regex;
use syn::{Attribute, Field, Fields, Type, TypePath};
use syn::Type::Path;
use crate::Parsed;

pub struct Model {
    pub struct_ident: Ident,
    pub struct_fields: Vec<FieldModel>
}

pub struct FieldModel {
    pub field_ident: Ident,
    pub field_ty: TokenStream,
    pub field_is_reserved: bool
}

pub fn analyze(parsed: Parsed) -> Model {
    let struct_ident = parsed.ident;
    let struct_fields: Vec<FieldModel> = match parsed.data_struct.fields {
        Fields::Named(fields) => fields.named.iter().map(analyze_field).collect(),
        _ => abort_call_site!("named field not found"), // we have already verified it is a struct with named fields
    };

    Model { struct_ident, struct_fields }
}

pub fn analyze_field(field: &Field) -> FieldModel {
    let field = field.clone();
    let field_ident = field.ident.unwrap();
    let field_str = field_ident.to_string();
    let field_attrs = field.attrs;
    let field_is_reserved = is_reserved_field_str(field_str, field_attrs);
    let field_ty = analyze_field_ty(&field.ty);

    FieldModel {
        field_ident,
        field_ty,
        field_is_reserved
    }
}

fn analyze_field_ty(field_ty: &Type) -> TokenStream {
    if is_modular_bitfield_type(field_ty) {
        return resolve_modular_bitfield_type(field_ty);
    } else if is_bilge_type(field_ty) {
        return resolve_bilge_type(field_ty);
    }

    field_ty.to_token_stream()
}

#[inline]
pub fn resolve_modular_bitfield_type(ty: &Type) -> TokenStream {
    quote!(<#ty as modular_bitfield::Specifier>::InOut)
}

#[inline]
pub fn resolve_bilge_type(ty: &Type) -> TokenStream {
    quote!(<<#ty as bilge::Bitsized>::ArbitraryInt as Number>::UnderlyingType)
}

pub fn analyze_type_ident(ty: &Type) -> Ident {
    match ty.clone() {
        Path(TypePath { path, .. }) => path.get_ident().expect("Expecting Ident in Type").clone(),
        _ => unreachable!("type not provided"),
    }
}

#[inline]
pub fn analyze_type_str(ty: &Type) -> String {
    analyze_type_ident(ty).to_string()
}

pub fn is_modular_bitfield_type(ty: &Type) -> bool {
    let ty_str = analyze_type_str(ty);

    if ty_str.starts_with('B') {
        let ty_bits = ty_str[1..].parse::<u8>();
        if ty_bits.is_ok() && ty_bits.unwrap() <= 128 {
            return true;
        }
    }

    false
}

pub fn is_bilge_type(ty: &Type) -> bool {
    let ty_str = analyze_type_str(ty);

    // note: look-around, including look-ahead and look-behind, is not supported
    let ty_regex = Regex::new(r"^u(1[0-2][0-7]|[1-9]|1[0-7]|2[0-9]|3[0-1]|5[0-9]|6[0-3]|7[0-9]|9[0-7])$");
    match ty_regex {
        Ok(re) => re.is_match(&ty_str),
        Err(err) => abort_call_site!("could not compile regex for bilge types: {}", err)
    }
}

pub fn is_reserved_field_str(str: String, attrs: Vec<Attribute>) -> bool {
    let modular_bitfield_reserved =
        str == "__" || attrs.iter().any(|attr| { attr.path().is_ident("skip") });

    let bilge_reserved =
        str == "reserved" || str == "padding" || str.contains("reserved_") || str.contains("padding_");

    modular_bitfield_reserved || bilge_reserved
}