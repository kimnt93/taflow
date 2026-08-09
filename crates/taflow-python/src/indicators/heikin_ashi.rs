use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::HeikinAshi as HeikinAshiState;

/// Python boundary for the canonical Rust Heikin-Ashi state.
#[pyclass]
pub struct HeikinAshi {
    inner: HeikinAshiState,
    open: Vec<f64>,
    high: Vec<f64>,
    low: Vec<f64>,
    close: Vec<f64>,
}

#[pymethods]
impl HeikinAshi {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: HeikinAshiState::new()
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            open: Vec::new(),
            high: Vec::new(),
            low: Vec::new(),
            close: Vec::new(),
        })
    }

    fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> (f64, f64, f64, f64) {
        let value = self.inner.append(open, high, low, close);
        self.open.push(value.open);
        self.high.push(value.high);
        self.low.push(value.low);
        self.close.push(value.close);
        (value.open, value.high, value.low, value.close)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        open: PyReadonlyArray1<f64>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (open, high, low, close) = (
            open.as_slice()?,
            high.as_slice()?,
            low.as_slice()?,
            close.as_slice()?,
        );
        let inner = &mut self.inner;
        let open_output = &mut self.open;
        let high_output = &mut self.high;
        let low_output = &mut self.low;
        let close_output = &mut self.close;
        py.allow_threads(|| {
            inner.extend_slices_into(
                open,
                high,
                low,
                close,
                open_output,
                high_output,
                low_output,
                close_output,
            )
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
        Bound<'py, PyArray1<f64>>,
    ) {
        (
            PyArray1::from_vec(py, self.open.clone()),
            PyArray1::from_vec(py, self.high.clone()),
            PyArray1::from_vec(py, self.low.clone()),
            PyArray1::from_vec(py, self.close.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.open, value.high, value.low, value.close))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.open.clear();
        self.high.clear();
        self.low.clear();
        self.close.clear();
    }

    fn __len__(&self) -> usize {
        self.open.len()
    }
}
