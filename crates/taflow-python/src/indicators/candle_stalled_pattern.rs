use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::CandleStalledPattern as CandleStalledPatternState;
#[pyclass]
/// Stateful CandleStalledPattern candlestick recognizer.
/// Inputs are OHLC bars; output is the aligned integer pattern score.
pub struct CandleStalledPattern {
    inner: CandleStalledPatternState,
    outputs: Vec<i32>,
}
#[pymethods]
impl CandleStalledPattern {
    #[new]
    fn new() -> Self {
        Self {
            inner: CandleStalledPatternState::new(),
            outputs: vec![],
        }
    }
    fn append(&mut self, o: f64, h: f64, l: f64, c: f64) -> Option<i32> {
        let v = self.inner.append(o, h, l, c);
        self.outputs.push(v.unwrap_or(0));
        v
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        o: PyReadonlyArray1<f64>,
        h: PyReadonlyArray1<f64>,
        l: PyReadonlyArray1<f64>,
        c: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (o, h, l, c) = (o.as_slice()?, h.as_slice()?, l.as_slice()?, c.as_slice()?);
        if o.len() != h.len() || o.len() != l.len() || o.len() != c.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        let outputs = &mut self.outputs;
        py.allow_threads(|| self.inner.extend_slices_into(o, h, l, c, outputs))
            .map_err(|error| PyValueError::new_err(error.to_string()))?;
        Ok(())
    }
    fn compute<'a>(&self, py: Python<'a>) -> Bound<'a, PyArray1<i32>> {
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
