use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{StandardErrorBands as State, StandardErrorBandsValue};

#[pyclass]
pub struct StandardErrorBands {
    inner: State,
    upper: Vec<f64>,
    middle: Vec<f64>,
    lower: Vec<f64>,
}

#[pymethods]
impl StandardErrorBands {
    #[new]
    fn new(period: usize, multiplier: f64) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(period, multiplier)
                .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))?,
            upper: Vec::new(),
            middle: Vec::new(),
            lower: Vec::new(),
        })
    }
    fn append(&mut self, input: f64) -> Option<(f64, f64, f64)> {
        let result = self.inner.append(input);
        let value = result.unwrap_or(StandardErrorBandsValue {
            upper: f64::NAN,
            middle: f64::NAN,
            lower: f64::NAN,
        });
        self.upper.push(value.upper);
        self.middle.push(value.middle);
        self.lower.push(value.lower);
        result.map(|value| (value.upper, value.middle, value.lower))
    }
    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        py.allow_threads(|| {
            for &value in input {
                self.append(value);
            }
        });
        Ok(())
    }
    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
    ) {
        (
            PyArray1::from_vec(py, self.upper.clone()),
            PyArray1::from_vec(py, self.middle.clone()),
            PyArray1::from_vec(py, self.lower.clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.upper, value.middle, value.lower))
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.upper.clear();
        self.middle.clear();
        self.lower.clear();
    }
    fn __len__(&self) -> usize {
        self.upper.len()
    }
}
