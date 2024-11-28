use std::string::String;

use pyo3::{pymethods, PyResult, wrap_pymodule};
use pyo3::prelude::*;

use crate::format::Format;
use crate::format::qubo::Qubo;

mod utils;
mod format;

#[pymethods]
impl Qubo {
    #[new]
    #[pyo3(signature = (size = 10, seed = 42))]
    fn new(size: usize, seed: u64) -> Self {
        Self::random(size, seed)
    }

    fn to_json_string(&self) -> String {
        Self::to_json_str(self)
    }

    #[staticmethod]
    fn from_json_string(json_str: String) -> Qubo {
        Qubo::from_json_str(json_str)
    }
}

#[pymodule]
#[pyo3(name = "generators")]
fn opt_generators(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Qubo>()?;

    Ok(())
}