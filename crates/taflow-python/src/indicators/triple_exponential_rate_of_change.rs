//! Python lifecycle for TRIX.

use crate::conversion::to_py_array;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{
    StreamingIndicator, TripleExponentialRateOfChange as NativeTripleExponentialRateOfChange,
};

#[pyclass]
pub struct TripleExponentialRateOfChange {
    inner: NativeTripleExponentialRateOfChange,
    outputs: Vec<f64>,
    timeperiod: usize,
}

#[pymethods]
impl TripleExponentialRateOfChange {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: NativeTripleExponentialRateOfChange::new(timeperiod)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            outputs: Vec::new(),
            timeperiod,
        })
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
    #[getter]
    fn timeperiod(&self) -> usize {
        self.timeperiod
    }
    fn __len__(&self) -> usize {
        self.outputs.len()
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}
