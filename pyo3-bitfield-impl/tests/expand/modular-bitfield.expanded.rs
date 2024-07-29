#[macro_use]
extern crate pyo3_bitfield_impl;
use modular_bitfield::prelude::*;
#[bitfield(bits = 96)]
pub struct Bitfield {
    #[skip]
    __: B15,
    status: B1,
    measurement_id: B16,
    timestamp: B64,
}
#[pyo3::pymethods]
impl Bitfield {
    ///Get the value of `status` in Python.
    fn py_get_status(&self) -> <B1 as modular_bitfield::Specifier>::InOut {
        self.status()
    }
    ///Get the value of `status` in Python.
    fn py_set_status(&mut self, value: <B1 as modular_bitfield::Specifier>::InOut) {
        self.set_status(value)
    }
    ///Get the value of `measurement_id` in Python.
    fn py_get_measurement_id(&self) -> <B16 as modular_bitfield::Specifier>::InOut {
        self.measurement_id()
    }
    ///Get the value of `measurement_id` in Python.
    fn py_set_measurement_id(
        &mut self,
        value: <B16 as modular_bitfield::Specifier>::InOut,
    ) {
        self.set_measurement_id(value)
    }
    ///Get the value of `timestamp` in Python.
    fn py_get_timestamp(&self) -> <B64 as modular_bitfield::Specifier>::InOut {
        self.timestamp()
    }
    ///Get the value of `timestamp` in Python.
    fn py_set_timestamp(&mut self, value: <B64 as modular_bitfield::Specifier>::InOut) {
        self.set_timestamp(value)
    }
}
