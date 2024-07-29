#[macro_use]
extern crate pyo3_bitfield_impl;

use bilge::prelude::*;

#[derive(FromBits, PyBitfield)]
pub struct Bilge {
    reserved: u15,
    status: u1,
    measurement_id: u16,
    timestamp: u64
}