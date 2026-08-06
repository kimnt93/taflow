//! Python lifecycle for the Commodity Channel Index (CCI).

use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::Cci;

use crate::conversion::to_py_array;

/// Persistent CCI with aligned HLC bulk input and scalar continuation.
#[pyclass]
pub struct CommodityChannelIndex {
    inner: Cci,
    outputs: Vec<f64>,
    timeperiod: usize,
}

#[pymethods]
impl CommodityChannelIndex {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Cci::new(timeperiod)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            outputs: Vec::new(),
            timeperiod,
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let value = self.inner.append(high, low, close);
        self.outputs.push(value.unwrap_or(f64::NAN));
        value
    }

    fn extend(
        &mut self,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let values = self
            .inner
            .extend_slice(high.as_slice()?, low.as_slice()?, close.as_slice()?)
            .map_err(|error| PyValueError::new_err(error.to_string()))?;
        self.outputs
            .extend(values.into_iter().map(|value| value.unwrap_or(f64::NAN)));
        Ok(())
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    #[getter]
    fn timeperiod(&self) -> usize {
        self.timeperiod
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}
