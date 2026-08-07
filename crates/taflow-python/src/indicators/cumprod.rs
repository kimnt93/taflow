use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::stream::CumulativeProduct;

#[pyclass]
pub struct CumulativeProductOperator { inner: CumulativeProduct, outputs: Vec<f64> }

#[pymethods]
impl CumulativeProductOperator {
    #[new]
    fn new() -> Self { Self { inner: CumulativeProduct::new(), outputs: Vec::new() } }
    fn append(&mut self, input: f64) -> f64 { let value = self.inner.append(input); self.outputs.push(value); value }
    fn extend(&mut self, input: PyReadonlyArray1<f64>) -> PyResult<()> { for &value in input.as_slice()? { self.append(value); } Ok(()) }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> { PyArray1::from_vec(py, self.outputs.clone()) }
    #[getter] fn value(&self) -> Option<f64> { self.inner.value() }
    fn reset(&mut self) { self.inner.reset(); self.outputs.clear(); }
}
