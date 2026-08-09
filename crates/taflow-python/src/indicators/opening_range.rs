use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::stream::{OpeningRange as State, OpeningRangeValue};

#[pyclass]
pub struct OpeningRange {
    inner: State,
    high: Vec<f64>,
    low: Vec<f64>,
    breakout: Vec<i32>,
}

#[pymethods]
impl OpeningRange {
    #[new]
    #[pyo3(signature = (bars=30))]
    fn new(bars: usize) -> Self {
        Self {
            inner: State::new(bars),
            high: Vec::new(),
            low: Vec::new(),
            breakout: Vec::new(),
        }
    }
    fn append(&mut self, high: f64, low: f64, close: f64, anchor: bool) -> (f64, f64, i32) {
        let value = self.inner.append(high, low, close, anchor);
        self.high.push(value.high);
        self.low.push(value.low);
        self.breakout.push(value.breakout);
        (value.high, value.low, value.breakout)
    }
    fn extend(
        &mut self,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
        anchor: PyReadonlyArray1<bool>,
    ) -> PyResult<()> {
        let (high, low, close, anchor) = (
            high.as_slice()?,
            low.as_slice()?,
            close.as_slice()?,
            anchor.as_slice()?,
        );
        let (inner, output_high, output_low, breakout) = (
            &mut self.inner,
            &mut self.high,
            &mut self.low,
            &mut self.breakout,
        );
        inner
            .extend_slice_into(high, low, close, anchor, output_high, output_low, breakout)
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))
    }
    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<i32>>,
    ) {
        (
            PyArray1::from_vec(py, self.high.clone()),
            PyArray1::from_vec(py, self.low.clone()),
            PyArray1::from_vec(py, self.breakout.clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64, i32)> {
        self.inner
            .value()
            .map(|value: OpeningRangeValue| (value.high, value.low, value.breakout))
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.high.clear();
        self.low.clear();
        self.breakout.clear();
    }
    fn __len__(&self) -> usize {
        self.high.len()
    }
}
