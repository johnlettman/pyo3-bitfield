#[macro_use]
extern crate pyo3_bitfield;
use modular_bitfield::prelude::*;
#[bitfield(bits = 96)]
pub struct Bitfield {
    #[skip]
    __: B15,
    status: B1,
    measurement_id: B16,
    timestamp: B64,
}
#[automatically_derived]
impl ::core::fmt::Debug for Bitfield {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "Bitfield",
            "__",
            &self.__,
            "status",
            &self.status,
            "measurement_id",
            &self.measurement_id,
            "timestamp",
            &&self.timestamp,
        )
    }
}
#[pymethods]
impl Bitfield {
    ///Get the value of `status` using Python.
    #[getter(status)]
    fn py_get_status(&self) -> <B1 as modular_bitfield::Specifier>::InOut {
        self.status()
    }
    ///Set the value of `status` using Python.
    #[setter(status)]
    fn py_set_status(&mut self, status: <B1 as modular_bitfield::Specifier>::InOut) {
        self.set_status(status)
    }
    ///Get the value of `measurement_id` using Python.
    #[getter(measurement_id)]
    fn py_get_measurement_id(&self) -> <B16 as modular_bitfield::Specifier>::InOut {
        self.measurement_id()
    }
    ///Set the value of `measurement_id` using Python.
    #[setter(measurement_id)]
    fn py_set_measurement_id(
        &mut self,
        measurement_id: <B16 as modular_bitfield::Specifier>::InOut,
    ) {
        self.set_measurement_id(measurement_id)
    }
    ///Get the value of `timestamp` using Python.
    #[getter(timestamp)]
    fn py_get_timestamp(&self) -> <B64 as modular_bitfield::Specifier>::InOut {
        self.timestamp()
    }
    ///Set the value of `timestamp` using Python.
    #[setter(timestamp)]
    fn py_set_timestamp(
        &mut self,
        timestamp: <B64 as modular_bitfield::Specifier>::InOut,
    ) {
        self.set_timestamp(timestamp)
    }
}
