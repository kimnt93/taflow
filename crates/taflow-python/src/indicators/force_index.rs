use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::ForceIndex;

#[pyclass]
pub struct ForceIndexOperator {
    inner: ForceIndex,
    outputs: Vec<f64>,
}

#[pymethods]
impl ForceIndexOperator {
    #[new]
    fn new() -> Self {
        Self {
            inner: ForceIndex::new(),
            outputs: Vec::new(),
        }
    }
    fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        let value = self.inner.append(close, volume);
        self.outputs.push(value.unwrap_or(f64::NAN));
        value
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        close: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (c, v) = (close.as_slice()?, volume.as_slice()?);
        if c.len() != v.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        py.allow_threads(|| {
            for (&c, &v) in c.iter().zip(v) {
                self.append(c, v);
            }
        });
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.outputs.clone())
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
