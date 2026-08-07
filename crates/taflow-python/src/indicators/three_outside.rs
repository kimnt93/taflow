use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::Candle3Outside;
#[pyclass]
/// Stateful CandleThreeOutside candlestick recognizer.
/// Inputs are OHLC bars; output is the aligned integer pattern score.
pub struct CandleThreeOutside {
    inner: Candle3Outside,
    outputs: Vec<i32>,
}
#[pymethods]
impl CandleThreeOutside {
    #[new]
    fn new() -> Self {
        Self {
            inner: Candle3Outside::new(),
            outputs: Vec::new(),
        }
    }
    fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let v = self.inner.append(open, high, low, close);
        self.outputs.push(v.unwrap_or(0));
        v
    }
    fn extend(
        &mut self,
        open: PyReadonlyArray1<f64>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (o, h, l, c) = (
            open.as_slice()?,
            high.as_slice()?,
            low.as_slice()?,
            close.as_slice()?,
        );
        if o.len() != h.len() || o.len() != l.len() || o.len() != c.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        for ((&o, &h), (&l, &c)) in o.iter().zip(h).zip(l.iter().zip(c)) {
            self.append(o, h, l, c);
        }
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
