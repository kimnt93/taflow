use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::OnBalanceVolume as State;

#[pyclass]
pub struct OnBalanceVolume {
    inner: State,
    outputs: Vec<f64>,
}

#[pymethods]
impl OnBalanceVolume {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new().map_err(|e| PyValueError::new_err(e.to_string()))?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, close: f64, volume: f64) -> f64 {
        let value = self.inner.append(close, volume);
        self.outputs.push(value);
        value
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        close: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (close, volume) = (close.as_slice()?, volume.as_slice()?);
        if close.len() != volume.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        py.allow_threads(|| {
            for (&c, &v) in close.iter().zip(volume) {
                self.append(c, v);
            }
        });
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.outputs.clone())
    }
    fn __len__(&self) -> usize {
        self.outputs.len()
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
