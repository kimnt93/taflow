use crate::conversion::to_py_array;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::PlusDirectionalIndicator as NativePlusDirectionalIndicator;
#[pyclass]
pub struct PlusDirectionalIndicator {
    inner: NativePlusDirectionalIndicator,
    outputs: Vec<f64>,
}
#[pymethods]
impl PlusDirectionalIndicator {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: NativePlusDirectionalIndicator::new(timeperiod).map_err(|e| PyValueError::new_err(e.to_string()))?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let v = self.inner.append(high, low, close);
        self.outputs.push(v.unwrap_or(f64::NAN));
        v
    }
    fn extend(
        &mut self,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let h = high.as_slice()?;
        let l = low.as_slice()?;
        let c = close.as_slice()?;
        if h.len() != l.len() || h.len() != c.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        self.outputs.extend(
            h.iter()
                .zip(l)
                .zip(c)
                .map(|((&h, &l), &c)| self.inner.append(h, l, c).unwrap_or(f64::NAN)),
        );
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
        self.outputs.clear();
    }
}
