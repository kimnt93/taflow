use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::VolumeZoneOscillator as State;
#[pyclass]
pub struct VolumeZoneOscillator {
    inner: State,
    output: Vec<f64>,
}
#[pymethods]
impl VolumeZoneOscillator {
    #[new]
    #[pyo3(signature=(timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(timeperiod).map_err(|e| PyValueError::new_err(e.to_string()))?,
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
        let (close, volume) = (close.as_slice()?, volume.as_slice()?);
        if close.len() != volume.len() {
            return Err(PyValueError::new_err(
                "close and volume must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            for (&c, &v) in close.iter().zip(volume) {
                self.append(c, v);
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
