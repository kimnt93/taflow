use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::AnchoredVolumeWeightedAveragePrice as AnchoredVolumeWeightedAveragePriceState;

/// Python boundary for the canonical Rust anchored VWAP state.
#[pyclass]
pub struct AnchoredVolumeWeightedAveragePrice {
    inner: AnchoredVolumeWeightedAveragePriceState,
    average: Vec<f64>,
    upper: Vec<f64>,
    lower: Vec<f64>,
}

#[pymethods]
impl AnchoredVolumeWeightedAveragePrice {
    #[new]
    #[pyo3(signature = (standard_deviation_multiplier=1.0))]
    fn new(standard_deviation_multiplier: f64) -> PyResult<Self> {
        Ok(Self {
            inner: AnchoredVolumeWeightedAveragePriceState::new(standard_deviation_multiplier)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            average: Vec::new(),
            upper: Vec::new(),
            lower: Vec::new(),
        })
    }

    fn append(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        anchor: bool,
    ) -> (f64, f64, f64) {
        let value = self.inner.append(high, low, close, volume, anchor);
        self.average.push(value.volume_weighted_average_price);
        self.upper.push(value.upper_band);
        self.lower.push(value.lower_band);
        (
            value.volume_weighted_average_price,
            value.upper_band,
            value.lower_band,
        )
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
        anchor: PyReadonlyArray1<bool>,
    ) -> PyResult<()> {
        let (high, low, close, volume, anchor) = (
            high.as_slice()?,
            low.as_slice()?,
            close.as_slice()?,
            volume.as_slice()?,
            anchor.as_slice()?,
        );
        let inner = &mut self.inner;
        let average = &mut self.average;
        let upper = &mut self.upper;
        let lower = &mut self.lower;
        py.allow_threads(|| {
            inner.extend_slices_into(high, low, close, volume, anchor, average, upper, lower)
        })
        .map_err(|error| PyValueError::new_err(error.to_string()))
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
            PyArray1::from_vec(py, self.average.clone()),
            PyArray1::from_vec(py, self.upper.clone()),
            PyArray1::from_vec(py, self.lower.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner.value().map(|value| {
            (
                value.volume_weighted_average_price,
                value.upper_band,
                value.lower_band,
            )
        })
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.average.clear();
        self.upper.clear();
        self.lower.clear();
    }

    fn __len__(&self) -> usize {
        self.average.len()
    }
}
