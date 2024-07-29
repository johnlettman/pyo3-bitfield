use pyo3_bitfield_impl::PyBitfield;

/// Attempt the [pyo3_bitfield_impl::PyBitfield] derive on an unsupported enum type.
#[derive(PyBitfield)]
enum TestEnum {
    Item1,
    Item2,
    Item3,
    Item4
}

fn main() {}