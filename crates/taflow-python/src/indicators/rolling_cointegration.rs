use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::{RollingCointegration as State, RollingCointegrationValue};

#[pyclass]
pub struct RollingCointegration {
    inner: State,
    hedge_ratio: Vec<f64>,
    spread: Vec<f64>,
    augmented_dickey_fuller_statistic: Vec<f64>,
}

#[pymethods]
impl RollingCointegration {
    #[new]
    fn new(period: usize, augmented_dickey_fuller_lags: usize) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(period, augmented_dickey_fuller_lags)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            hedge_ratio: Vec::new(),
            spread: Vec::new(),
            augmented_dickey_fuller_statistic: Vec::new(),
        })
    }
    fn append(&mut self, left: f64, right: f64) -> Option<(f64, f64, f64)> {
        let result = self.inner.append(left, right);
        let value = result.unwrap_or(RollingCointegrationValue {
            hedge_ratio: f64::NAN,
            spread: f64::NAN,
            augmented_dickey_fuller_statistic: f64::NAN,
        });
        self.hedge_ratio.push(value.hedge_ratio);
        self.spread.push(value.spread);
        self.augmented_dickey_fuller_statistic
            .push(value.augmented_dickey_fuller_statistic);
        result.map(|value| {
            (
                value.hedge_ratio,
                value.spread,
                value.augmented_dickey_fuller_statistic,
            )
        })
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        left: PyReadonlyArray1<f64>,
        right: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (left, right) = (left.as_slice()?, right.as_slice()?);
        if left.len() != right.len() {
            return Err(PyValueError::new_err(
                "left and right inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            for index in 0..left.len() {
                self.append(left[index], right[index]);
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
            PyArray1::from_vec(py, self.hedge_ratio.clone()),
            PyArray1::from_vec(py, self.spread.clone()),
            PyArray1::from_vec(py, self.augmented_dickey_fuller_statistic.clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner.value().map(|value| {
            (
                value.hedge_ratio,
                value.spread,
                value.augmented_dickey_fuller_statistic,
            )
        })
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.hedge_ratio.clear();
        self.spread.clear();
        self.augmented_dickey_fuller_statistic.clear();
    }
    fn __len__(&self) -> usize {
        self.hedge_ratio.len()
    }
}
