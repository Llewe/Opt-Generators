use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;


mod utils;
mod format;

use format::qubo;
use crate::format::Format;
use crate::format::qubo::Qubo;

#[pymethods]
impl Qubo {
    #[new]
    fn new(size: usize, seed: u64) -> Self {
        Self::random(size, seed)
    }

    fn to_json_str(&self) -> String{
        Self::to_string(self)
    }
}

#[pymodule]
fn opt_formats(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Qubo>()?;
    Ok(())
}