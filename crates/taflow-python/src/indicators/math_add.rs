use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::MathAdd as State;
#[pyclass]
pub struct MathAdd {
    inner: State,
    output: Vec<f64>,
}
#[pymethods]
impl MathAdd {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new().unwrap(),
            output: Vec::new(),
        })
    }
    fn append(&mut self, left: f64, right: f64) -> f64 {
        let v = self.inner.append(left, right);
        self.output.push(v);
        v
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        left: PyReadonlyArray1<f64>,
        right: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let l = left.as_slice()?;
        let r = right.as_slice()?;
        if l.len() != r.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        py.allow_threads(|| self.inner.extend_slices_into(l, r, &mut self.output))
            .map_err(|e| PyValueError::new_err(e.to_string()))
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
        self.output.clear()
    }
    fn __len__(&self) -> usize {
        self.output.len()
    }
}
