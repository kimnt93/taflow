use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{KlingerVolumeOscillator as State, KlingerVolumeOscillatorValue};

/// Python boundary for the canonical Rust Klinger Volume Oscillator state.
#[pyclass]
pub struct KlingerVolumeOscillator {
    inner: State,
    oscillator: Vec<f64>,
    signal: Vec<f64>,
}

#[pymethods]
impl KlingerVolumeOscillator {
    #[new]
    #[pyo3(signature = (fast=34, slow=55, signal=13))]
    fn new(fast: usize, slow: usize, signal: usize) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(fast, slow, signal)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            oscillator: Vec::new(),
            signal: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<(f64, f64)> {
        let value = self.inner.append(high, low, close, volume);
        let (oscillator, signal) = self.inner.outputs();
        self.oscillator.push(oscillator);
        self.signal.push(signal);
        value.map(|value| (value.oscillator, value.signal))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (high, low, close, volume) = (
            high.as_slice()?,
            low.as_slice()?,
            close.as_slice()?,
            volume.as_slice()?,
        );
        let (inner, oscillator, signal) = (&mut self.inner, &mut self.oscillator, &mut self.signal);
        py.allow_threads(|| {
            inner
                .extend_slice_into(high, low, close, volume, oscillator, signal)
                .map_err(|error| PyValueError::new_err(error.to_string()))
        })
    }

    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
        (
            PyArray1::from_vec(py, self.oscillator.clone()),
            PyArray1::from_vec(py, self.signal.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner
            .value()
            .map(|value: KlingerVolumeOscillatorValue| (value.oscillator, value.signal))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.oscillator.clear();
        self.signal.clear();
    }

    fn __len__(&self) -> usize {
        self.oscillator.len()
    }
}
