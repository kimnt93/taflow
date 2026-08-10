use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::VolumeOscillator as State;
#[pyclass]
pub struct VolumeOscillator {
    inner: State,
    output: Vec<f64>,
}
#[pymethods]
impl VolumeOscillator {
    #[new]
    #[pyo3(signature=(fast=5,slow=10))]
    fn new(fast: usize, slow: usize) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(fast, slow).map_err(|e| PyValueError::new_err(e.to_string()))?,
            output: Vec::new(),
        })
    }
    fn append(&mut self, input: f64) -> Option<f64> {
        let v = self.inner.append(input);
        self.output.push(v.unwrap_or(f64::NAN));
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
