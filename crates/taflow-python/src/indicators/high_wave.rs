use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::CandleHighWave as CandleHighWaveState;
#[pyclass]
/// Stateful CandleHighWave candlestick recognizer.
/// Inputs are OHLC bars; output is the aligned integer pattern score.
pub struct CandleHighWave {
    inner: CandleHighWaveState,
    outputs: Vec<i32>,
}
#[pymethods]
impl CandleHighWave {
    #[new]
    fn new() -> Self {
        Self {
            inner: CandleHighWaveState::new(),
            outputs: Vec::new(),
        }
    }
    fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let value = self.inner.append(open, high, low, close);
        self.outputs.push(value.unwrap_or(0));
        value
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        open: PyReadonlyArray1<f64>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let open = open.as_slice()?;
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        if open.len() != high.len() || open.len() != low.len() || open.len() != close.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        let outputs = &mut self.outputs;
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(open, high, low, close, outputs)
        })
        .map_err(|error| PyValueError::new_err(error.to_string()))?;
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<i32>> {
        PyArray1::from_vec(py, self.outputs.clone())
    }
    #[getter]
    fn value(&self) -> Option<i32> {
        self.inner.value()
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}
