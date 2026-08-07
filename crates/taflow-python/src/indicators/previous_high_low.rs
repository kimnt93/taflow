use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::PreviousHighLow;

#[pyclass]
pub struct PreviousHighLowOperator {
    inner: PreviousHighLow,
    prev_high: Vec<f64>,
    prev_low: Vec<f64>,
    broken_high: Vec<f64>,
    broken_low: Vec<f64>,
}

#[pymethods]
impl PreviousHighLowOperator {
    #[new]
    fn new() -> Self {
        Self {
            inner: PreviousHighLow::new(),
            prev_high: Vec::new(),
            prev_low: Vec::new(),
            broken_high: Vec::new(),
            broken_low: Vec::new(),
        }
    }

    fn append(&mut self, new_session: bool, high: f64, low: f64) -> (f64, f64, f64, f64) {
        let value = self.inner.append(new_session, high, low);
        self.prev_high.push(value.prev_high);
        self.prev_low.push(value.prev_low);
        self.broken_high.push(value.broken_high);
        self.broken_low.push(value.broken_low);
        (
            value.prev_high,
            value.prev_low,
            value.broken_high,
            value.broken_low,
        )
    }

    fn extend(
        &mut self,
        new_session: PyReadonlyArray1<bool>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (new_session, high, low) = (new_session.as_slice()?, high.as_slice()?, low.as_slice()?);
        if new_session.len() != high.len() || high.len() != low.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        for ((&new_session, &high), &low) in new_session.iter().zip(high).zip(low) {
            self.append(new_session, high, low);
        }
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
            PyArray1::from_vec(py, self.prev_high.clone()),
            PyArray1::from_vec(py, self.prev_low.clone()),
            PyArray1::from_vec(py, self.broken_high.clone()),
            PyArray1::from_vec(py, self.broken_low.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64, f64)> {
        self.inner.value().map(|value| {
            (
                value.prev_high,
                value.prev_low,
                value.broken_high,
                value.broken_low,
            )
        })
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.prev_high.clear();
        self.prev_low.clear();
        self.broken_high.clear();
        self.broken_low.clear();
    }
}
