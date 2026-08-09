use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::Supertrend;

#[pyclass]
pub struct SupertrendOperator {
    inner: Supertrend,
    trend: Vec<f64>,
    direction: Vec<f64>,
    long: Vec<f64>,
    short: Vec<f64>,
}

#[pymethods]
impl SupertrendOperator {
    #[new]
    #[pyo3(signature = (timeperiod=7, multiplier=3.0))]
    fn new(timeperiod: usize, multiplier: f64) -> PyResult<Self> {
        Ok(Self {
            inner: Supertrend::new(timeperiod, multiplier)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            trend: Vec::new(),
            direction: Vec::new(),
            long: Vec::new(),
            short: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<(f64, f64, f64, f64)> {
        let value = self.inner.append(high, low, close);
        match value {
            Some(value) => {
                self.trend.push(value.trend);
                self.direction.push(value.direction);
                self.long.push(value.long);
                self.short.push(value.short);
                Some((value.trend, value.direction, value.long, value.short))
            }
            None => {
                self.trend.push(f64::NAN);
                self.direction.push(f64::NAN);
                self.long.push(f64::NAN);
                self.short.push(f64::NAN);
                None
            }
        }
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
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        py.allow_threads(|| {
            for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
                self.append(high, low, close);
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
        Bound<'py, PyArray1<f64>>,
    ) {
        (
            PyArray1::from_vec(py, self.trend.clone()),
            PyArray1::from_vec(py, self.direction.clone()),
            PyArray1::from_vec(py, self.long.clone()),
            PyArray1::from_vec(py, self.short.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.trend, value.direction, value.long, value.short))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.trend.clear();
        self.direction.clear();
        self.long.clear();
        self.short.clear();
    }
}
