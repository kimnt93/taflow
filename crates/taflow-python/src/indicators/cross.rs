use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::stream::Cross;

#[pyclass]
pub struct CrossOperator {
    inner: Cross,
    outputs: Vec<f64>,
}

#[pymethods]
impl CrossOperator {
    #[new]
    fn new() -> Self {
        Self {
            inner: Cross::new(),
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
        if left.len() != right.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            left.iter().zip(right).for_each(|(&a, &b)| {
                self.append(a, b);
            })
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
    fn __len__(&self) -> usize {
        self.outputs.len()
    }
}
