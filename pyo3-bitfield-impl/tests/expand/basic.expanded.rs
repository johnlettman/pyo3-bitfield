#[macro_use]
extern crate pyo3_bitfield_impl;
#[bitfield]
#[pyclass]
pub struct Basic {
    identifier: u16,
    timestamp: u64,
}
#[pyo3::pymethods]
impl Basic {
    ///Get the value of `identifier` in Python.
    fn py_get_identifier(&self) -> u16 {
        self.identifier()
    }
    ///Get the value of `identifier` in Python.
    fn py_set_identifier(&mut self, value: u16) {
        self.set_identifier(value)
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
