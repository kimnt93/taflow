use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::SmoothedTrendChannel as State;

#[pyclass]
pub struct SmoothedTrendChannel {
    inner: State,
    lower: Vec<f64>,
    upper: Vec<f64>,
}

#[pymethods]
impl SmoothedTrendChannel {
    #[new]
    #[pyo3(signature = (length=10))]
    fn new(length: usize) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(length).map_err(|e| PyValueError::new_err(e.to_string()))?,
            lower: Vec::new(),
            upper: Vec::new(),
        })
    }
    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<(f64, f64)> {
        let value = self.inner.append(high, low, close);
        match value {
            Some((lower, upper)) => {
                self.lower.push(lower);
                self.upper.push(upper);
            }
            None => {
                self.lower.push(f64::NAN);
                self.upper.push(f64::NAN);
            }
        }
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
        let (inner, lower, upper) = (&mut self.inner, &mut self.lower, &mut self.upper);
        py.allow_threads(|| {
            inner
                .extend_slice_into(high, low, close, lower, upper)
                .map_err(|e| PyValueError::new_err(e.to_string()))
        })
    }
    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
        (
            PyArray1::from_vec(py, self.lower.clone()),
            PyArray1::from_vec(py, self.upper.clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value()
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.lower.clear();
        self.upper.clear();
    }
    fn __len__(&self) -> usize {
        self.lower.len()
    }
}
