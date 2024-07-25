#[macro_use]
extern crate pyo3_bitfield;
#[bitfield]
#[pyclass]
pub struct Basic {
    identifier: u16,
    timestamp: u64,
}
#[pymethods]
impl Basic {
    ///Get the value of `identifier` using Python.
    #[getter(identifier)]
    fn py_get_identifier(&self) -> u16 {
        self.identifier()
    }
    ///Set the value of `identifier` using Python.
    #[setter(identifier)]
    fn py_set_identifier(&mut self, identifier: u16) {
        self.set_identifier(identifier)
    }
    ///Get the value of `timestamp` using Python.
    #[getter(timestamp)]
    fn py_get_timestamp(&self) -> u64 {
        self.timestamp()
    }
    ///Set the value of `timestamp` using Python.
    #[setter(timestamp)]
    fn py_set_timestamp(&mut self, timestamp: u64) {
        self.set_timestamp(timestamp)
    }
}
