use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::Squeeze;

#[pyclass]
pub struct SqueezeOperator {
    inner: Squeeze,
    squeeze: Vec<f64>,
    on: Vec<f64>,
    off: Vec<f64>,
    no: Vec<f64>,
}

#[pymethods]
impl SqueezeOperator {
    #[new]
    #[pyo3(signature = (bb_length=20, bb_std=2.0, kc_length=20, kc_scalar=1.5, mom_length=12, mom_smooth=6))]
    fn new(
        bb_length: usize,
        bb_std: f64,
        kc_length: usize,
        kc_scalar: f64,
        mom_length: usize,
        mom_smooth: usize,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: Squeeze::new(
                bb_length,
                bb_std,
                kc_length,
                kc_scalar,
                mom_length,
                mom_smooth,
            )
            .map_err(|error| PyValueError::new_err(error.to_string()))?,
            squeeze: Vec::new(),
            on: Vec::new(),
            off: Vec::new(),
            no: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> (f64, f64, f64, f64) {
        let value = self.inner.append(high, low, close);
        self.squeeze.push(value.squeeze);
        self.on.push(value.on);
        self.off.push(value.off);
        self.no.push(value.no);
        (value.squeeze, value.on, value.off, value.no)
    }

    fn extend(
        &mut self,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (high, low, close) = (high.as_slice()?, low.as_slice()?, close.as_slice()?);
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
            self.append(high, low, close);
        }
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
            PyArray1::from_vec(py, self.squeeze.clone()),
            PyArray1::from_vec(py, self.on.clone()),
            PyArray1::from_vec(py, self.off.clone()),
            PyArray1::from_vec(py, self.no.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64, f64)> {
        self.inner.value().map(|value| {
            (
                value.squeeze,
                value.on,
                value.off,
                value.no,
            )
        })
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.squeeze.clear();
        self.on.clear();
        self.off.clear();
        self.no.clear();
    }
}
