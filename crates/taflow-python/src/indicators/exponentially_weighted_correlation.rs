use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::ExponentiallyWeightedCorrelation as State;
#[pyclass]
pub struct ExponentiallyWeightedCorrelation {
    inner: State,
    outputs: Vec<f64>,
}
#[pymethods]
impl ExponentiallyWeightedCorrelation {
    #[new]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(timeperiod)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, left: f64, right: f64) -> f64 {
        let v = self.inner.append(left, right);
        self.outputs.push(v);
        v
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        left: PyReadonlyArray1<f64>,
        right: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (a, b) = (left.as_slice()?, right.as_slice()?);
        if a.len() != b.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        py.allow_threads(|| {
            for (&x, &y) in a.iter().zip(b) {
                self.append(x, y);
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
