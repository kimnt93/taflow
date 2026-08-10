use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::AdaptiveCycle as State;
#[pyclass]
pub struct AdaptiveCycle {
    inner: State,
    output: Vec<f64>,
}
#[pymethods]
impl AdaptiveCycle {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new().unwrap(),
            output: Vec::new(),
        })
    }
    fn append(&mut self, x: f64) -> Option<f64> {
        let v = self.inner.append(x);
        self.output.push(v.unwrap_or(f64::NAN));
        v
    }
    fn extend(&mut self, py: Python<'_>, x: PyReadonlyArray1<f64>) -> PyResult<()> {
        let x = x.as_slice()?;
        py.allow_threads(|| {
            for &a in x {
                self.append(a);
            }
        });
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
        self.output.clear();
    }
    fn __len__(&self) -> usize {
        self.output.len()
    }
}
