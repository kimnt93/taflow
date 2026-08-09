use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::Retracements;

#[pyclass]
pub struct RetracementsOperator {
    inner: Retracements,
    direction: Vec<f64>,
    current_retracement_pct: Vec<f64>,
    deepest_retracement_pct: Vec<f64>,
}

#[pymethods]
impl RetracementsOperator {
    #[new]
    #[pyo3(signature = (swing_length=5))]
    fn new(swing_length: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Retracements::new(swing_length)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            direction: Vec::new(),
            current_retracement_pct: Vec::new(),
            deepest_retracement_pct: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> (f64, f64, f64) {
        let value = self.inner.append(high, low, close);
        self.direction.push(value.direction);
        self.current_retracement_pct
            .push(value.current_retracement_pct);
        self.deepest_retracement_pct
            .push(value.deepest_retracement_pct);
        (
            value.direction,
            value.current_retracement_pct,
            value.deepest_retracement_pct,
        )
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (high, low, close) = (high.as_slice()?, low.as_slice()?, close.as_slice()?);
        if high.len() != low.len() || low.len() != close.len() {
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
    ) {
        (
            PyArray1::from_vec(py, self.direction.clone()),
            PyArray1::from_vec(py, self.current_retracement_pct.clone()),
            PyArray1::from_vec(py, self.deepest_retracement_pct.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner.value().map(|value| {
            (
                value.direction,
                value.current_retracement_pct,
                value.deepest_retracement_pct,
            )
        })
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.direction.clear();
        self.current_retracement_pct.clear();
        self.deepest_retracement_pct.clear();
    }
}
