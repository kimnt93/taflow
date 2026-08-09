use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::Vortex;

#[pyclass]
pub struct VortexOperator {
    inner: Vortex,
    vp: Vec<f64>,
    vn: Vec<f64>,
}

#[pymethods]
impl VortexOperator {
    #[new]
    #[pyo3(signature = (window=14))]
    fn new(window: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Vortex::new(window).map_err(|error| PyValueError::new_err(error.to_string()))?,
            vp: Vec::new(),
            vn: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> (f64, f64) {
        let value = self.inner.append(high, low, close);
        self.vp.push(value.vp);
        self.vn.push(value.vn);
        (value.vp, value.vn)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (high, low, close) = (high.as_slice()?, low.as_slice()?, close.as_slice()?);
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        py.allow_threads(|| {
            for ((&high, &low), &close) in high.iter().zip(low).zip(close) {
                self.append(high, low, close);
            }
        });
        Ok(())
    }

    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
        (
            PyArray1::from_vec(py, self.vp.clone()),
            PyArray1::from_vec(py, self.vn.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value().map(|value| (value.vp, value.vn))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.vp.clear();
        self.vn.clear();
    }
}
