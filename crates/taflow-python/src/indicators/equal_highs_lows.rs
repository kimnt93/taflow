use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::EqualHighsLows;

#[pyclass]
pub struct EqualHighsLowsOperator {
    inner: EqualHighsLows,
    eqh: Vec<f64>,
    eql: Vec<f64>,
    level: Vec<f64>,
}

#[pymethods]
impl EqualHighsLowsOperator {
    #[new]
    #[pyo3(signature = (eq_len=3, atr_period=200, eq_threshold=0.1))]
    fn new(eq_len: usize, atr_period: usize, eq_threshold: f64) -> PyResult<Self> {
        Ok(Self {
            inner: EqualHighsLows::new(eq_len, atr_period, eq_threshold)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            eqh: Vec::new(),
            eql: Vec::new(),
            level: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> (f64, f64, f64) {
        let value = self.inner.append(high, low, close);
        self.eqh.push(value.eqh);
        self.eql.push(value.eql);
        self.level.push(value.level);
        (value.eqh, value.eql, value.level)
    }

    fn extend(
        &mut self,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (high, low, close) = (high.as_slice()?, low.as_slice()?, close.as_slice()?);
        if high.len() != low.len() || low.len() != close.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
            self.append(high, low, close);
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
    ) {
        (
            PyArray1::from_vec(py, self.eqh.clone()),
            PyArray1::from_vec(py, self.eql.clone()),
            PyArray1::from_vec(py, self.level.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.eqh, value.eql, value.level))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.eqh.clear();
        self.eql.clear();
        self.level.clear();
    }
}
