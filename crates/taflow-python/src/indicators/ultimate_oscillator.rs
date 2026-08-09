//! Python lifecycle for the Ultimate Oscillator (ULTOSC).

use crate::conversion::to_py_array;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::UltimateOscillator as NativeUltimateOscillator;

#[pyclass]
pub struct UltimateOscillator {
    inner: NativeUltimateOscillator,
    outputs: Vec<f64>,
}
#[pymethods]
impl UltimateOscillator {
    #[new]
    #[pyo3(signature = (timeperiod1=7, timeperiod2=14, timeperiod3=28))]
    fn new(timeperiod1: usize, timeperiod2: usize, timeperiod3: usize) -> PyResult<Self> {
        Ok(Self {
            inner: NativeUltimateOscillator::new(timeperiod1, timeperiod2, timeperiod3)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let value = self.inner.append(high, low, close);
        self.outputs.push(value.unwrap_or(f64::NAN));
        value
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        let outputs = &mut self.outputs;
        py.allow_threads(|| self.inner.extend_slices_into(high, low, close, outputs))
            .map_err(|error| PyValueError::new_err(error.to_string()))?;
        Ok(())
    }
    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }
    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }
    fn __len__(&self) -> usize {
        self.outputs.len()
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}
