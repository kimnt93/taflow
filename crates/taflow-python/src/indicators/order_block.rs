use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::OrderBlock;

#[pyclass]
pub struct OrderBlockOperator {
    inner: OrderBlock,
    ob: Vec<f64>,
    top: Vec<f64>,
    bottom: Vec<f64>,
    ob_volume: Vec<f64>,
    mitigated: Vec<f64>,
}

#[pymethods]
impl OrderBlockOperator {
    #[new]
    #[pyo3(signature = (swing_length=50, internal_length=5, atr_period=200, threshold=2.0))]
    fn new(
        swing_length: usize,
        internal_length: usize,
        atr_period: usize,
        threshold: f64,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: OrderBlock::new(swing_length, internal_length, atr_period, threshold)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            ob: Vec::new(),
            top: Vec::new(),
            bottom: Vec::new(),
            ob_volume: Vec::new(),
            mitigated: Vec::new(),
        })
    }

    fn append(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
    ) -> (f64, f64, f64, f64, f64) {
        let value = self.inner.append(high, low, close, volume);
        self.ob.push(value.ob);
        self.top.push(value.top);
        self.bottom.push(value.bottom);
        self.ob_volume.push(value.ob_volume);
        self.mitigated.push(value.mitigated);
        (
            value.ob,
            value.top,
            value.bottom,
            value.ob_volume,
            value.mitigated,
        )
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (high, low, close, volume) = (
            high.as_slice()?,
            low.as_slice()?,
            close.as_slice()?,
            volume.as_slice()?,
        );
        if high.len() != low.len() || low.len() != close.len() || close.len() != volume.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        py.allow_threads(|| {
            for (((&high, &low), &close), &volume) in high.iter().zip(low).zip(close).zip(volume) {
                self.append(high, low, close, volume);
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
        Bound<'py, PyArray1<f64>>,
    ) {
        (
            PyArray1::from_vec(py, self.ob.clone()),
            PyArray1::from_vec(py, self.top.clone()),
            PyArray1::from_vec(py, self.bottom.clone()),
            PyArray1::from_vec(py, self.ob_volume.clone()),
            PyArray1::from_vec(py, self.mitigated.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64, f64, f64)> {
        self.inner.value().map(|value| {
            (
                value.ob,
                value.top,
                value.bottom,
                value.ob_volume,
                value.mitigated,
            )
        })
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.ob.clear();
        self.top.clear();
        self.bottom.clear();
        self.ob_volume.clear();
        self.mitigated.clear();
    }
}
