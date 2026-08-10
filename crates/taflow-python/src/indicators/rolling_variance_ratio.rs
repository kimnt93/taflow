use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::RollingVarianceRatio as State;
#[pyclass]
pub struct RollingVarianceRatio {
    inner: State,
    output: Vec<f64>,
}
#[pymethods]
impl RollingVarianceRatio {
    #[new]
    fn new(period: usize, q: usize) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(period, q).map_err(|e| PyValueError::new_err(e.to_string()))?,
            output: Vec::new(),
        })
    }
    fn append(&mut self, a: f64, b: f64) -> Option<f64> {
        let v = self.inner.append(a, b);
        self.output.push(v.unwrap_or(f64::NAN));
        v
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        a: PyReadonlyArray1<f64>,
        b: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (a, b) = (a.as_slice()?, b.as_slice()?);
        if a.len() != b.len() {
            return Err(PyValueError::new_err("a and b must have equal lengths"));
        }
        py.allow_threads(|| {
            for (&x, &y) in a.iter().zip(b) {
                self.append(x, y);
            }
        });
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.output.clone())
    }
    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.output.clear();
    }
    fn __len__(&self) -> usize {
        self.output.len()
    }
}
