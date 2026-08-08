use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::Sessions;

#[pyclass]
pub struct SessionsOperator {
    inner: Sessions,
    active: Vec<f64>,
    session_high: Vec<f64>,
    session_low: Vec<f64>,
}

#[pymethods]
impl SessionsOperator {
    #[new]
    fn new() -> Self {
        Self {
            inner: Sessions::new(),
            active: Vec::new(),
            session_high: Vec::new(),
            session_low: Vec::new(),
        }
    }

    fn append(&mut self, new_session: bool, high: f64, low: f64) -> (f64, f64, f64) {
        let value = self.inner.append(new_session, high, low);
        self.active.push(value.active);
        self.session_high.push(value.session_high);
        self.session_low.push(value.session_low);
        (value.active, value.session_high, value.session_low)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        new_session: PyReadonlyArray1<bool>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (new_session, high, low) = (new_session.as_slice()?, high.as_slice()?, low.as_slice()?);
        if new_session.len() != high.len() || high.len() != low.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        py.allow_threads(|| {
            for ((&new_session, &high), &low) in new_session.iter().zip(high).zip(low) {
                self.append(new_session, high, low);
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
            PyArray1::from_vec(py, self.active.clone()),
            PyArray1::from_vec(py, self.session_high.clone()),
            PyArray1::from_vec(py, self.session_low.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.active, value.session_high, value.session_low))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.active.clear();
        self.session_high.clear();
        self.session_low.clear();
    }
}
