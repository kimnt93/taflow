//! Python lifecycle for the Exponential Moving Average (EMA).

use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{
    ExponentialMovingAverage as NativeExponentialMovingAverage, StreamingIndicator,
};

use crate::conversion::to_py_array;

/// Persistent EMA with bulk initialization and O(1) scalar continuation.
#[pyclass]
pub struct ExponentialMovingAverage {
    inner: NativeExponentialMovingAverage,
    outputs: Vec<f64>,
    timeperiod: usize,
}

#[pymethods]
impl ExponentialMovingAverage {
    #[new]
    #[pyo3(signature = (timeperiod=30))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: NativeExponentialMovingAverage::new(timeperiod)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            outputs: Vec::new(),
            timeperiod,
        })
    }

    /// Processes one new value without revisiting prior observations.
    fn append(&mut self, input: f64) -> Option<f64> {
        let value = self.inner.append(input);
        self.outputs.push(value.unwrap_or(f64::NAN));
        value
    }

    /// Processes a contiguous chunk and retains both state and aligned output.
    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        self.outputs.reserve(input.len());
        let outputs = &mut self.outputs;
        let inner = &mut self.inner;
        py.allow_threads(|| {
            outputs.extend(
                inner
                    .extend_slice(input)
                    .into_iter()
                    .map(|value| value.unwrap_or(f64::NAN)),
            );
        });
        Ok(())
    }

    /// Materializes all accumulated values without recomputing them.
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

    /// Clears values and recurrence state while retaining allocated storage.
    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}
