use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::FastStochasticOscillator as State;
use taflow::MaType;

#[pyclass]
pub struct FastStochasticOscillator {
    inner: State,
    fastk: Vec<f64>,
    fastd: Vec<f64>,
}

#[pymethods]
impl FastStochasticOscillator {
    #[new]
    #[pyo3(signature = (fastk_period=5, fastd_period=3, fastd_matype=0))]
    fn new(fastk_period: usize, fastd_period: usize, fastd_matype: i32) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(
                fastk_period,
                fastd_period,
                MaType::try_from(fastd_matype).map_err(|e| PyValueError::new_err(e.to_string()))?,
            )
            .map_err(|e| PyValueError::new_err(e.to_string()))?,
            fastk: Vec::new(),
            fastd: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<(f64, f64)> {
        let value = self
            .inner
            .append(high, low, close)
            .map(|value| (value.fastk, value.fastd));
        let output = value.unwrap_or((f64::NAN, f64::NAN));
        self.fastk.push(output.0);
        self.fastd.push(output.1);
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
                .extend_slices_into(high, low, close, &mut self.fastk, &mut self.fastd)
                .map_err(|e| PyValueError::new_err(e.to_string()))
        })
    }

    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
        (
            PyArray1::from_vec(py, self.fastk.clone()),
            PyArray1::from_vec(py, self.fastd.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value().map(|value| (value.fastk, value.fastd))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.fastk.clear();
        self.fastd.clear();
    }
    fn __len__(&self) -> usize {
        self.fastk.len()
    }
}
