use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::VolumePriceTrend;

#[pyclass]
pub struct VptOperator { inner: VolumePriceTrend, values: Vec<f64> }

#[pymethods]
impl VptOperator {
    #[new]
    fn new() -> Self { Self { inner: VolumePriceTrend::new(), values: Vec::new() } }
    fn append(&mut self, close: f64, volume: f64) -> Option<f64> { let value = self.inner.append(close, volume); self.values.push(value.unwrap_or(f64::NAN)); value }
    fn extend(&mut self, close: PyReadonlyArray1<f64>, volume: PyReadonlyArray1<f64>) -> PyResult<()> {
        let (close, volume) = (close.as_slice()?, volume.as_slice()?);
        if close.len() != volume.len() { return Err(PyValueError::new_err("inputs must have equal lengths")); }
        for (&close, &volume) in close.iter().zip(volume) { self.append(close, volume); }
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> { PyArray1::from_vec(py, self.values.clone()) }
    #[getter] fn value(&self) -> Option<f64> { self.inner.value() }
    fn reset(&mut self) { self.inner.reset(); self.values.clear(); }
}
