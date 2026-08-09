use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{RollingVariance as State, StreamingIndicator};

#[pyclass]
pub struct RollingVariance {
    inner: State,
    output: Vec<f64>,
}

#[pymethods]
impl RollingVariance {
    #[new]
    #[pyo3(signature = (timeperiod=14, nbdev=1.0))]
    fn new(timeperiod: usize, nbdev: f64) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(timeperiod, nbdev)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            output: Vec::new(),
        })
    }
    fn append(&mut self, input: f64) -> Option<f64> {
        let value = self.inner.append(input);
        self.output.push(value.unwrap_or(f64::NAN));
        value
    }
    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        py.allow_threads(|| self.inner.extend_slice_into(input, &mut self.output));
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
