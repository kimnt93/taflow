use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{
    RollingLeadLagCrossCorrelation as State, RollingLeadLagCrossCorrelationValue,
};

#[pyclass]
pub struct RollingLeadLagCrossCorrelation {
    inner: State,
    lag: Vec<f64>,
    correlation: Vec<f64>,
}

#[pymethods]
impl RollingLeadLagCrossCorrelation {
    #[new]
    fn new(window: usize, max_lag: usize) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(window, max_lag)
                .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))?,
            lag: Vec::new(),
            correlation: Vec::new(),
        })
    }
    fn append(&mut self, left: f64, right: f64) -> Option<(f64, f64)> {
        let result = self.inner.append(left, right);
        let value = result.unwrap_or(RollingLeadLagCrossCorrelationValue {
            lag: f64::NAN,
            correlation: f64::NAN,
        });
        self.lag.push(value.lag);
        self.correlation.push(value.correlation);
        result.map(|value| (value.lag, value.correlation))
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        left: PyReadonlyArray1<f64>,
        right: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (left, right) = (left.as_slice()?, right.as_slice()?);
        if left.len() != right.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
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
    ) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
        (
            PyArray1::from_vec(py, self.lag.clone()),
            PyArray1::from_vec(py, self.correlation.clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.lag, value.correlation))
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.lag.clear();
        self.correlation.clear();
    }
    fn __len__(&self) -> usize {
        self.lag.len()
    }
}
