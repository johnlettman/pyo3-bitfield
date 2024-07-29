#[macro_use]
extern crate pyo3_bitfield_impl;
use bilge::prelude::*;
pub struct Bilge {
    reserved: u15,
    status: u1,
    measurement_id: u16,
    timestamp: u64,
}
#[pyo3::pymethods]
impl Bilge {
    ///Get the value of `status` in Python.
    fn py_get_status(
        &self,
    ) -> <<u1 as bilge::Bitsized>::ArbitraryInt as Number>::UnderlyingType {
        self.status()
    }
    ///Get the value of `status` in Python.
    fn py_set_status(
        &mut self,
        value: <<u1 as bilge::Bitsized>::ArbitraryInt as Number>::UnderlyingType,
    ) {
        self.set_status(value)
    }
    ///Get the value of `measurement_id` in Python.
    fn py_get_measurement_id(
        &self,
    ) -> <<u16 as bilge::Bitsized>::ArbitraryInt as Number>::UnderlyingType {
        self.measurement_id()
    }
    ///Get the value of `measurement_id` in Python.
    fn py_set_measurement_id(
        &mut self,
        value: <<u16 as bilge::Bitsized>::ArbitraryInt as Number>::UnderlyingType,
    ) {
        self.set_measurement_id(value)
    }
    ///Get the value of `timestamp` in Python.
    fn py_get_timestamp(&self) -> u64 {
        self.timestamp()
    }
    ///Get the value of `timestamp` in Python.
    fn py_set_timestamp(&mut self, value: u64) {
        self.set_timestamp(value)
    }
}
