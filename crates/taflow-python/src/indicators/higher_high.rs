use crate::state_api::py_value_error;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::stream::HigherHigh;
#[pyclass]
pub struct HigherHighOperator {
    inner: HigherHigh,
    outputs: Vec<f64>,
}
#[pymethods]
impl HigherHighOperator {
    #[new]
    fn new() -> Self {
        Self {
            inner: HigherHigh::new(),
            outputs: Vec::new(),
        }
    }
    fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let v = self.inner.append(high, low);
        self.outputs.push(v.unwrap_or(f64::NAN));
        v
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (high, low) = (high.as_slice()?, low.as_slice()?);
        py.allow_threads(|| self.inner.extend_slices_into(high, low, &mut self.outputs))
            .map_err(py_value_error)
    }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.outputs.clone())
    }
    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
    fn __len__(&self) -> usize {
        self.outputs.len()
    }
}
