use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::RollingCov;
#[pyclass]
pub struct RollingCovOperator {
    inner: RollingCov,
    outputs: Vec<f64>,
}
#[pymethods]
impl RollingCovOperator {
    #[new]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: RollingCov::new(timeperiod).map_err(|e| PyValueError::new_err(e.to_string()))?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, left: f64, right: f64) -> Option<f64> {
        let value = self.inner.append(left, right);
        self.outputs.push(value.unwrap_or(f64::NAN));
        value
    }
    fn extend(
        &mut self,
        left: PyReadonlyArray1<f64>,
        right: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (left, right) = (left.as_slice()?, right.as_slice()?);
        if left.len() != right.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        for (&left, &right) in left.iter().zip(right) {
            self.append(left, right);
        }
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
