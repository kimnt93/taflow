use crate::conversion::to_py_array;
use crate::state_api::{push_option, py_value_error};
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::RollingLinearRegressionIntercept as State;
use taflow::stream::StreamingIndicator;

#[pyclass]
pub struct RollingLinearRegressionIntercept {
    inner: State,
    outputs: Vec<f64>,
}

#[pymethods]
impl RollingLinearRegressionIntercept {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(timeperiod).map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, input: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(input))
    }
    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        py.allow_threads(|| self.inner.extend_slice_into(input, &mut self.outputs));
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
    fn __len__(&self) -> usize {
        self.outputs.len()
    }
}
