use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::VolumeRelativeStrengthIndex as State;
#[pyclass]
pub struct VolumeRelativeStrengthIndex {
    inner: State,
    output: Vec<f64>,
}
#[pymethods]
impl VolumeRelativeStrengthIndex {
    #[new]
    fn new(period: usize) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(period).map_err(|e| PyValueError::new_err(e.to_string()))?,
            output: Vec::new(),
        })
    }
    fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        let v = self.inner.append(close, volume);
        self.output.push(v.unwrap_or(f64::NAN));
        v
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        close: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (c, v) = (close.as_slice()?, volume.as_slice()?);
        if c.len() != v.len() {
            return Err(PyValueError::new_err(
                "close and volume must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            for (&x, &y) in c.iter().zip(v) {
                self.append(x, y);
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
