use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::SessionExtrema;

#[pyclass]
pub struct SessionExtremaOperator {
    inner: SessionExtrema,
    highs: Vec<f64>,
    lows: Vec<f64>,
}

#[pymethods]
impl SessionExtremaOperator {
    #[new]
    fn new() -> Self {
        Self { inner: SessionExtrema::new(), highs: Vec::new(), lows: Vec::new() }
    }

    fn append(&mut self, new_session: bool, high: f64, low: f64) -> (f64, f64) {
        let value = self.inner.append(new_session, high, low);
        self.highs.push(value.high);
        self.lows.push(value.low);
        (value.high, value.low)
    }

    fn extend(
        &mut self,
        new_session: PyReadonlyArray1<bool>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (new_session, high, low) =
            (new_session.as_slice()?, high.as_slice()?, low.as_slice()?);
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
    ) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
        (
            PyArray1::from_vec(py, self.highs.clone()),
            PyArray1::from_vec(py, self.lows.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value().map(|value| (value.high, value.low))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.highs.clear();
        self.lows.clear();
    }
}
