use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::MathCos as State;
#[pyclass]
pub struct MathCos {
    inner: State,
    output: Vec<f64>,
}
#[pymethods]
impl MathCos {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new().unwrap(),
            output: Vec::new(),
        })
    }
    fn append(&mut self, input: f64) -> f64 {
        let v = self.inner.append(input).unwrap();
        self.output.push(v);
        v
    }
    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let i = input.as_slice()?;
        py.allow_threads(|| self.inner.extend_slice_into(i, &mut self.output));
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
        self.output.clear()
    }
    fn __len__(&self) -> usize {
        self.output.len()
    }
}
