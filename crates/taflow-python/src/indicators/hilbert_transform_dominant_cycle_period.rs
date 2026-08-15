use crate::conversion::to_py_array;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::HilbertTransformDominantCyclePeriod as NativeHilbertTransformDominantCyclePeriod;

#[pyclass]
pub struct HilbertTransformDominantCyclePeriod {
    inner: NativeHilbertTransformDominantCyclePeriod,
    outputs: Vec<f64>,
}

#[pymethods]
impl HilbertTransformDominantCyclePeriod {
    #[new]
    fn new() -> Self {
        Self {
            inner: NativeHilbertTransformDominantCyclePeriod::new(),
            outputs: Vec::new(),
        }
    }
    fn append(&mut self, input: f64) -> Option<f64> {
        let value = self.inner.append(input);
        self.outputs.push(value.unwrap_or(f64::NAN));
        value
    }
    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        let outputs = &mut self.outputs;
        let inner = &mut self.inner;
        py.allow_threads(|| inner.extend_slice_into(input, outputs));
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
