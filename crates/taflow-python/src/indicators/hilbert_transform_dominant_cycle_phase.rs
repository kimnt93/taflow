use crate::conversion::to_py_array;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::stream::HtDcphase;

#[pyclass]
pub struct HilbertTransformDominantCyclePhase {
    inner: HtDcphase,
    outputs: Vec<f64>,
}

#[pymethods]
impl HilbertTransformDominantCyclePhase {
    #[new]
    fn new() -> Self {
        Self {
            inner: HtDcphase::new(),
            outputs: Vec::new(),
        }
    }
    fn append(&mut self, input: f64) -> Option<f64> {
        let value = self.inner.append(input);
        self.outputs.push(value.unwrap_or(f64::NAN));
        value
    }
    fn extend(&mut self, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        for &input in input.as_slice()? {
            self.append(input);
        }
        Ok(())
    }
    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }
    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}
