use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{HurstChannel as State, HurstChannelValue};

#[pyclass]
pub struct HurstChannel {
    inner: State,
    upper: Vec<f64>,
    middle: Vec<f64>,
    lower: Vec<f64>,
}

#[pymethods]
impl HurstChannel {
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
    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<(f64, f64, f64)> {
        let result = self.inner.append(high, low, close);
        let value = result.unwrap_or(HurstChannelValue {
            upper: f64::NAN,
            middle: f64::NAN,
            lower: f64::NAN,
        });
        self.upper.push(value.upper);
        self.middle.push(value.middle);
        self.lower.push(value.lower);
        result.map(|value| (value.upper, value.middle, value.lower))
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (high, low, close) = (high.as_slice()?, low.as_slice()?, close.as_slice()?);
        if high.len() != low.len() || high.len() != close.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "high, low, and close inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            for index in 0..high.len() {
                self.append(high[index], low[index], close[index]);
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
