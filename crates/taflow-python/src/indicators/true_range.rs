use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::TrueRange as State;

#[pyclass]
pub struct TrueRange {
    inner: State,
    outputs: Vec<f64>,
}

#[pymethods]
impl TrueRange {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new().map_err(|e| PyValueError::new_err(e.to_string()))?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let value = self.inner.append(high, low, close);
        self.outputs.push(value.unwrap_or(f64::NAN));
        value
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (high, low, close) = (high.as_slice()?, low.as_slice()?, close.as_slice()?);
        if [low.len(), close.len()]
            .iter()
            .any(|&len| len != high.len())
        {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(high, low, close, &mut self.outputs)
        })
        .map_err(|error| PyValueError::new_err(error.to_string()))?;
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.outputs.clone())
    }
    fn __len__(&self) -> usize {
        self.outputs.len()
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
