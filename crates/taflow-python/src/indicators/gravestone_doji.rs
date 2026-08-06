use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::CdlGravestoneDoji;
#[pyclass]
pub struct GravestoneDoji {
    inner: CdlGravestoneDoji,
    outputs: Vec<i32>,
}
#[pymethods]
impl GravestoneDoji {
    #[new]
    fn new() -> Self {
        Self {
            inner: CdlGravestoneDoji::new(),
            outputs: Vec::new(),
        }
    }
    fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let value = self.inner.append(open, high, low, close);
        self.outputs.push(value.unwrap_or(0));
        value
    }
    fn extend(
        &mut self,
        open: PyReadonlyArray1<f64>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let open = open.as_slice()?;
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        if open.len() != high.len() || open.len() != low.len() || open.len() != close.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        for ((&open, &high), (&low, &close)) in open.iter().zip(high).zip(low.iter().zip(close)) {
            self.append(open, high, low, close);
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
