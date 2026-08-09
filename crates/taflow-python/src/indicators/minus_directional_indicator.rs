use crate::conversion::to_py_array;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::MinusDirectionalIndicator as NativeMinusDirectionalIndicator;
#[pyclass]
pub struct MinusDirectionalIndicator {
    inner: NativeMinusDirectionalIndicator,
    outputs: Vec<f64>,
}
#[pymethods]
impl MinusDirectionalIndicator {
    #[new]
    #[pyo3(signature=(timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: NativeMinusDirectionalIndicator::new(timeperiod)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, h: f64, l: f64, c: f64) -> Option<f64> {
        let v = self.inner.append(h, l, c);
        self.outputs.push(v.unwrap_or(f64::NAN));
        v
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        h: PyReadonlyArray1<f64>,
        l: PyReadonlyArray1<f64>,
        c: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let h = h.as_slice()?;
        let l = l.as_slice()?;
        let c = c.as_slice()?;
        if h.len() != l.len() || h.len() != c.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        py.allow_threads(|| self.inner.extend_slices_into(h, l, c, &mut self.outputs))
            .map_err(|error| PyValueError::new_err(error.to_string()))?;
        Ok(())
    }
    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }
    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear()
    }
}
