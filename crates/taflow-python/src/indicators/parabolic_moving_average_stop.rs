use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::{ParabolicMovingAverageStop as State, ParabolicMovingAverageStopValue};

/// Python boundary for the canonical Rust PMAX state.
#[pyclass]
pub struct ParabolicMovingAverageStop {
    inner: State,
    stops: Vec<f64>,
    trends: Vec<i32>,
}

#[pymethods]
impl ParabolicMovingAverageStop {
    #[new]
    #[pyo3(signature = (length=10, multiplier=3.0))]
    fn new(length: usize, multiplier: f64) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(length, multiplier)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            stops: Vec::new(),
            trends: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<(f64, i32)> {
        let value = self.inner.append(high, low, close);
        if let Some(value) = value {
            self.stops.push(value.stop);
            self.trends.push(value.trend);
            Some((value.stop, value.trend))
        } else {
            self.stops.push(f64::NAN);
            self.trends.push(0);
            None
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
        let (inner, stops, trends) = (&mut self.inner, &mut self.stops, &mut self.trends);
        py.allow_threads(|| {
            inner
                .extend_slice_into(high, low, close, stops, trends)
                .map_err(|error| PyValueError::new_err(error.to_string()))
        })
    }

    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<i32>>) {
        (
            PyArray1::from_vec(py, self.stops.clone()),
            PyArray1::from_vec(py, self.trends.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, i32)> {
        self.inner
            .value()
            .map(|value: ParabolicMovingAverageStopValue| (value.stop, value.trend))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.stops.clear();
        self.trends.clear();
    }

    fn __len__(&self) -> usize {
        self.stops.len()
    }
}
