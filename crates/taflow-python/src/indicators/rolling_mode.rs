use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::RollingMode;

#[pyclass]
pub struct RollingModeOperator { inner: RollingMode, outputs: Vec<f64> }

#[pymethods]
impl RollingModeOperator {
    #[new]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self { inner: RollingMode::new(timeperiod).map_err(|e| PyValueError::new_err(e.to_string()))?, outputs: Vec::new() })
    }
    fn append(&mut self, input: f64) -> Option<f64> { let value = self.inner.append(input); self.outputs.push(value.unwrap_or(f64::NAN)); value }
    fn extend(&mut self, input: PyReadonlyArray1<f64>) -> PyResult<()> { for &value in input.as_slice()? { self.append(value); } Ok(()) }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> { PyArray1::from_vec(py, self.outputs.clone()) }
    #[getter] fn value(&self) -> Option<f64> { self.inner.value() }
    fn reset(&mut self) { self.inner.reset(); self.outputs.clear(); }
}
