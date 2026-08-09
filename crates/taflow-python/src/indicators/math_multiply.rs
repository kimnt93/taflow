use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::MathMultiply as State;

#[pyclass]
pub struct MathMultiply {
    inner: State,
    outputs: Vec<f64>,
}

#[pymethods]
impl MathMultiply {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new().map_err(crate::state_api::py_value_error)?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, left: f64, right: f64) -> f64 {
        let value = self.inner.append(left, right);
        self.outputs.push(value);
        value
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        left: PyReadonlyArray1<f64>,
        right: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (l, r) = (left.as_slice()?, right.as_slice()?);
        let inner = &mut self.inner;
        let outputs = &mut self.outputs;
        py.allow_threads(|| inner.extend_slices_into(l, r, outputs))
            .map_err(crate::state_api::py_value_error)
    }
    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.outputs.clone())
    }
    fn __len__(&self) -> usize {
        self.outputs.len()
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}
