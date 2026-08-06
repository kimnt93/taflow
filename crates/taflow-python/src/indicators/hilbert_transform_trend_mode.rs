use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::stream::HtTrendmode;

#[pyclass]
pub struct HilbertTransformTrendMode {
    inner: HtTrendmode,
    outputs: Vec<i32>,
}
#[pymethods]
impl HilbertTransformTrendMode {
    #[new]
    fn new() -> Self {
        Self {
            inner: HtTrendmode::new(),
            outputs: Vec::new(),
        }
    }
    fn append(&mut self, input: f64) -> Option<i32> {
        let value = self.inner.append(input);
        self.outputs.push(value.unwrap_or(0));
        value
    }
    fn extend(&mut self, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        for &input in input.as_slice()? {
            self.append(input);
        }
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<i32>> {
        PyArray1::from_vec(py, self.outputs.clone())
    }
    #[getter]
    fn value(&self) -> Option<i32> {
        self.inner.value()
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}
