use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::BarsSince;

#[pyclass]
pub struct BarsSinceOperator {
    inner: BarsSince,
    outputs: Vec<f64>,
}

#[pymethods]
impl BarsSinceOperator {
    #[new]
    fn new() -> Self {
        Self {
            inner: BarsSince::new(),
            outputs: Vec::new(),
        }
    }
    fn append(&mut self, condition: bool) -> Option<f64> {
        let value = self.inner.append(condition);
        self.outputs.push(value.unwrap_or(f64::NAN));
        value
    }
    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<bool>) -> PyResult<()> {
        let input = input.as_slice()?;
        py.allow_threads(|| {
            input.iter().for_each(|&value| {
                self.append(value);
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
