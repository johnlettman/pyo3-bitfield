use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use crate::FieldModel;
use crate::Model;

pub struct Intermediate {
    pub struct_ident: Ident,
    pub fields: Vec<TokenStream>,
}

pub fn generate(model: Model) -> Intermediate {
    let Model { struct_ident, struct_fields } = model;

    let fields = struct_fields
        .iter()
        .filter(|field_model| {!field_model.field_is_reserved})
        .map(generate_field).collect();

    Intermediate{struct_ident, fields}
}

pub fn generate_field(field_model: &FieldModel) -> TokenStream {
    let FieldModel { field_ident, field_ty, .. } = field_model;

    let field_getter_doc = generate_getter_doc(&field_ident);
    let field_setter_doc = generate_setter_doc(&field_ident);

    let field_getter_fn = generate_getter_fn_ident(&field_ident);
    let field_setter_fn = generate_setter_fn_ident(&field_ident);

    let field_target_getter_fn = generate_target_getter_fn_ident(&field_ident);
    let field_target_setter_fn = generate_target_setter_fn_ident(&field_ident);

    quote!{
        #[doc = #field_getter_doc]
        fn #field_getter_fn(&self) -> #field_ty {
            self.#field_target_getter_fn()
        }

        #[doc = #field_setter_doc]
        fn #field_setter_fn(&mut self, value: #field_ty) {
            self.#field_target_setter_fn(value)
        }
    }
}

#[inline]
pub fn generate_getter_doc(field_ident: &Ident) -> String {
    format!("Get the value of `{field_ident}` in Python.")
}

#[inline]
pub fn generate_setter_doc(field_ident: &Ident) -> String {
    format!("Get the value of `{field_ident}` in Python.")
}

#[inline]
pub fn generate_getter_fn_ident(field_ident: &Ident) -> Ident {
    format_ident!("py_get_{field_ident}")
}

#[inline]
pub fn generate_setter_fn_ident(field_ident: &Ident) -> Ident {
    format_ident!("py_set_{field_ident}")
}

#[inline]
pub fn generate_target_getter_fn_ident(field_ident: &Ident) -> Ident {
    format_ident!("{field_ident}")
}

#[inline]
pub fn generate_target_setter_fn_ident(field_ident: &Ident) -> Ident {
    format_ident!("set_{field_ident}")
}