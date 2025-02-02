#[macro_use]
extern crate pyo3_bitfield_impl;

#[bitfield]
#[pyclass]
#[derive(PyBitfield)]
pub struct Basic {
    identifier: u16,
    timestamp: u64
}
