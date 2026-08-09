use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::FairValueGap;

#[pyclass]
pub struct FairValueGapOperator {
    inner: FairValueGap,
    signal: Vec<f64>,
    top: Vec<f64>,
    bottom: Vec<f64>,
    mitigated: Vec<f64>,
}

#[pymethods]
impl FairValueGapOperator {
    #[new]
    fn new() -> Self {
        Self {
            inner: FairValueGap::new(),
            signal: Vec::new(),
            top: Vec::new(),
            bottom: Vec::new(),
            mitigated: Vec::new(),
        }
    }

    fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> (f64, f64, f64, f64) {
        let value = self
            .inner
            .append(open, high, low, close)
            .expect("FVG always emits an aligned value");
        self.signal.push(value.signal);
        self.top.push(value.top);
        self.bottom.push(value.bottom);
        self.mitigated.push(value.mitigated);
        (value.signal, value.top, value.bottom, value.mitigated)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        open: PyReadonlyArray1<f64>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (open, high, low, close) = (
            open.as_slice()?,
            high.as_slice()?,
            low.as_slice()?,
            close.as_slice()?,
        );
        if open.len() != high.len() || high.len() != low.len() || low.len() != close.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        py.allow_threads(|| {
            for (((&open, &high), &low), &close) in open.iter().zip(high).zip(low).zip(close) {
                self.append(open, high, low, close);
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
        Bound<'py, PyArray1<f64>>,
    ) {
        (
            PyArray1::from_vec(py, self.signal.clone()),
            PyArray1::from_vec(py, self.top.clone()),
            PyArray1::from_vec(py, self.bottom.clone()),
            PyArray1::from_vec(py, self.mitigated.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.signal, value.top, value.bottom, value.mitigated))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.signal.clear();
        self.top.clear();
        self.bottom.clear();
        self.mitigated.clear();
    }

    fn __len__(&self) -> usize {
        self.signal.len()
    }
}
