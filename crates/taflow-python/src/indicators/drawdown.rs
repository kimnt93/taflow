use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::Drawdown as State;
#[pyclass]
pub struct Drawdown {
    inner: State,
    outputs: Vec<f64>,
}
#[pymethods]
impl Drawdown {
    #[new]
    fn new() -> Self {
        Self {
            inner: State::new(),
            outputs: Vec::new(),
        }
    }
    fn append(&mut self, input: f64) -> f64 {
        let v = self.inner.append(input);
        self.outputs.push(v);
        v
    }
    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        py.allow_threads(|| {
            for &v in input {
                self.append(v);
            }
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
}
