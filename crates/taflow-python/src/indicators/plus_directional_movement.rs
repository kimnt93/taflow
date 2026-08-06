use crate::conversion::to_py_array;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::stream::PlusDm;
#[pyclass]
pub struct PlusDirectionalMovement {
    inner: PlusDm,
    outputs: Vec<f64>,
}
#[pymethods]
impl PlusDirectionalMovement {
    #[new]
    #[pyo3(signature=(timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: PlusDm::new(timeperiod)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, h: f64, l: f64) -> Option<f64> {
        let v = self.inner.append(h, l);
        self.outputs.push(v.unwrap_or(f64::NAN));
        v
    }
    fn extend(&mut self, h: PyReadonlyArray1<f64>, l: PyReadonlyArray1<f64>) -> PyResult<()> {
        let h = h.as_slice()?;
        let l = l.as_slice()?;
        if h.len() != l.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "inputs must have equal lengths",
            ));
        }
        self.outputs.extend(
            h.iter()
                .zip(l)
                .map(|(&h, &l)| self.inner.append(h, l).unwrap_or(f64::NAN)),
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
        self.outputs.clear()
    }
}
