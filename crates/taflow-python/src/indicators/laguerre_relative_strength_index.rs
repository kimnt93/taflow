use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{
    LaguerreRelativeStrengthIndex as LaguerreRelativeStrengthIndexState, StreamingIndicator,
};

/// Python boundary for the canonical Rust Laguerre Relative Strength Index state.
#[pyclass]
pub struct LaguerreRelativeStrengthIndex {
    inner: LaguerreRelativeStrengthIndexState,
    output: Vec<f64>,
}

#[pymethods]
impl LaguerreRelativeStrengthIndex {
    #[new]
    #[pyo3(signature = (gamma=0.5))]
    fn new(gamma: f64) -> PyResult<Self> {
        Ok(Self {
            inner: LaguerreRelativeStrengthIndexState::new(gamma)
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
