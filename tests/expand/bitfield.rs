#[macro_use]
extern crate pyo3_bitfield;

use modular_bitfield::prelude::*;

#[bitfield(bits = 96)]
#[derive(Debug, PyBitfield)]
pub struct Bitfield {
    #[skip] __: B15,
    status: B1,
    measurement_id: B16,
    timestamp: B64
}