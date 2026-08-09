use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{StochasticOscillator as State, StreamingIndicator};
use taflow::MaType;

#[pyclass]
pub struct StochasticOscillator {
    inner: State,
    slowk: Vec<f64>,
    slowd: Vec<f64>,
}

#[pymethods]
impl StochasticOscillator {
    #[new]
    #[pyo3(signature = (fastk_period=5, slowk_period=3, slowk_matype=0, slowd_period=3, slowd_matype=0))]
    fn new(
        fastk_period: usize,
        slowk_period: usize,
        slowk_matype: i32,
        slowd_period: usize,
        slowd_matype: i32,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(
                fastk_period,
                slowk_period,
                MaType::try_from(slowk_matype).map_err(|e| PyValueError::new_err(e.to_string()))?,
                slowd_period,
                MaType::try_from(slowd_matype).map_err(|e| PyValueError::new_err(e.to_string()))?,
            )
            .map_err(|e| PyValueError::new_err(e.to_string()))?,
            slowk: Vec::new(),
            slowd: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<(f64, f64)> {
        let value = self
            .inner
            .append(high, low, close)
            .map(|value| (value.slowk, value.slowd));
        let output = value.unwrap_or((f64::NAN, f64::NAN));
        self.slowk.push(output.0);
        self.slowd.push(output.1);
        value
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let high = high.as_slice()?;
        let low = low.as_slice()?;
        let close = close.as_slice()?;
        if high.len() != low.len() || high.len() != close.len() {
            return Err(PyValueError::new_err(
                "high, low, and close must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(high, low, close, &mut self.slowk, &mut self.slowd)
                .map_err(|e| PyValueError::new_err(e.to_string()))
        })
    }

    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
        (
            PyArray1::from_vec(py, self.slowk.clone()),
            PyArray1::from_vec(py, self.slowd.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value().map(|value| (value.slowk, value.slowd))
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.slowk.clear();
        self.slowd.clear();
    }
    fn __len__(&self) -> usize {
        self.slowk.len()
    }
}
