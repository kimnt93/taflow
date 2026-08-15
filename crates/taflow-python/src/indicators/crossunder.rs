use crate::state_api::py_value_error;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::stream::Crossunder;

#[pyclass]
pub struct CrossunderOperator {
    inner: Crossunder,
    outputs: Vec<f64>,
}

#[pymethods]
impl CrossunderOperator {
    #[new]
    fn new() -> Self {
        Self {
            inner: Crossunder::new(),
            outputs: Vec::new(),
        }
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
        let (left, right) = (left.as_slice()?, right.as_slice()?);
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(left, right, &mut self.outputs)
        })
        .map_err(py_value_error)
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
    fn __len__(&self) -> usize {
        self.outputs.len()
    }
}
