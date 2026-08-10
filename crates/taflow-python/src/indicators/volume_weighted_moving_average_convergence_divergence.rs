use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{
    VolumeWeightedMovingAverageConvergenceDivergence as State,
    VolumeWeightedMovingAverageConvergenceDivergenceValue as Value,
};

#[pyclass]
pub struct VolumeWeightedMovingAverageConvergenceDivergence {
    inner: State,
    convergence_divergence: Vec<f64>,
    signal: Vec<f64>,
    histogram: Vec<f64>,
}

#[pymethods]
impl VolumeWeightedMovingAverageConvergenceDivergence {
    #[new]
    fn new(fast: usize, slow: usize, signal: usize) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(fast, slow, signal)
                .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))?,
            convergence_divergence: Vec::new(),
            signal: Vec::new(),
            histogram: Vec::new(),
        })
    }
    fn append(&mut self, close: f64, volume: f64) -> Option<(f64, f64, f64)> {
        let result = self.inner.append(close, volume);
        let value = result.unwrap_or(Value {
            convergence_divergence: f64::NAN,
            signal: f64::NAN,
            histogram: f64::NAN,
        });
        self.convergence_divergence
            .push(value.convergence_divergence);
        self.signal.push(value.signal);
        self.histogram.push(value.histogram);
        result.map(|value| (value.convergence_divergence, value.signal, value.histogram))
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        close: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (close, volume) = (close.as_slice()?, volume.as_slice()?);
        if close.len() != volume.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "close and volume inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            for index in 0..close.len() {
                self.append(close[index], volume[index]);
            }
        });
        Ok(())
    }
    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
    ) {
        (
            PyArray1::from_vec(py, self.convergence_divergence.clone()),
            PyArray1::from_vec(py, self.signal.clone()),
            PyArray1::from_vec(py, self.histogram.clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.convergence_divergence, value.signal, value.histogram))
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.convergence_divergence.clear();
        self.signal.clear();
        self.histogram.clear();
    }
    fn __len__(&self) -> usize {
        self.convergence_divergence.len()
    }
}
