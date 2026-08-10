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
    #[pyo3(signature=(period=14))]
    fn new(period: usize) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(period).map_err(|error| PyValueError::new_err(error.to_string()))?,
            output: Vec::new(),
        })
    }

    fn append(&mut self, volume: f64) -> Option<f64> {
        let value = self.inner.append(volume);
        self.output.push(value.unwrap_or(f64::NAN));
        value
    }

    fn extend(&mut self, py: Python<'_>, volume: PyReadonlyArray1<f64>) -> PyResult<()> {
        let volume = volume.as_slice()?;
        py.allow_threads(|| {
            for &value in volume {
                self.append(value);
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
