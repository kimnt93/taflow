use crate::conversion::to_py_array;
use crate::state_api::py_value_error;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::MedianPrice as State;

#[pyclass]
pub struct MedianPrice {
    inner: State,
    outputs: Vec<f64>,
}

#[pymethods]
impl MedianPrice {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new().map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, high: f64, low: f64) -> f64 {
        let value = self.inner.append(high, low);
        self.outputs.push(value);
        value
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        py.allow_threads(|| self.inner.extend_slices_into(high, low, &mut self.outputs))
            .map_err(py_value_error)
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
    fn __len__(&self) -> usize {
        self.outputs.len()
    }
}
