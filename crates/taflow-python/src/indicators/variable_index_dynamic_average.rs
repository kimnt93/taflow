use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::VariableIndexDynamicAverage as VariableIndexDynamicAverageState;
use taflow::stream::StreamingIndicator;

/// Python boundary for the canonical Rust Variable Index Dynamic Average state.
#[pyclass]
pub struct VariableIndexDynamicAverage {
    inner: VariableIndexDynamicAverageState,
    output: Vec<f64>,
}

#[pymethods]
impl VariableIndexDynamicAverage {
    #[new]
    #[pyo3(signature = (length=14, alpha=None))]
    fn new(length: usize, alpha: Option<f64>) -> PyResult<Self> {
        let alpha = alpha.unwrap_or(2.0 / (length as f64 + 1.0));
        Ok(Self {
            inner: VariableIndexDynamicAverageState::new(length, alpha)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            output: Vec::new(),
        })
    }

    fn append(&mut self, close: f64) -> Option<f64> {
        let value = self.inner.append(close);
        self.output.push(value.unwrap_or(f64::NAN));
        value
    }

    fn extend(&mut self, py: Python<'_>, close: PyReadonlyArray1<f64>) -> PyResult<()> {
        let close = close.as_slice()?;
        let inner = &mut self.inner;
        let output = &mut self.output;
        py.allow_threads(|| inner.extend_slice_into(close, output));
        Ok(())
    }

    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.output.clone())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.output.clear();
    }

    fn __len__(&self) -> usize {
        self.output.len()
    }
}
