use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::ChaikinMoneyFlow;

#[pyclass]
pub struct CmfOperator { inner: ChaikinMoneyFlow, values: Vec<f64> }

#[pymethods]
impl CmfOperator {
    #[new]
    #[pyo3(signature = (period=20))]
    fn new(period: usize) -> PyResult<Self> {
        Ok(Self { inner: ChaikinMoneyFlow::new(period).map_err(|error| PyValueError::new_err(error.to_string()))?, values: Vec::new() })
    }

    fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        let value = self.inner.append(high, low, close, volume);
        self.values.push(value.unwrap_or(f64::NAN));
        value
    }

    fn extend(&mut self, high: PyReadonlyArray1<f64>, low: PyReadonlyArray1<f64>, close: PyReadonlyArray1<f64>, volume: PyReadonlyArray1<f64>) -> PyResult<()> {
        let (high, low, close, volume) = (high.as_slice()?, low.as_slice()?, close.as_slice()?, volume.as_slice()?);
        if high.len() != low.len() || high.len() != close.len() || high.len() != volume.len() { return Err(PyValueError::new_err("inputs must have equal lengths")); }
        for (((&h, &l), &c), &v) in high.iter().zip(low).zip(close).zip(volume) { self.append(h, l, c, v); }
        Ok(())
    }

    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> { PyArray1::from_vec(py, self.values.clone()) }

    #[getter]
    fn value(&self) -> Option<f64> { self.inner.value() }

    fn reset(&mut self) { self.inner.reset(); self.values.clear(); }
}
