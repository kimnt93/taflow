use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::Stc;

#[pyclass]
pub struct StcOperator {
    inner: Stc,
    stc: Vec<f64>,
    macd: Vec<f64>,
    stoch: Vec<f64>,
}

#[pymethods]
impl StcOperator {
    #[new]
    #[pyo3(signature = (tclength=10, fast=12, slow=26, factor=0.5))]
    fn new(tclength: usize, fast: usize, slow: usize, factor: f64) -> PyResult<Self> {
        Ok(Self {
            inner: Stc::new(tclength, fast, slow, factor)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            stc: Vec::new(),
            macd: Vec::new(),
            stoch: Vec::new(),
        })
    }

    fn append(&mut self, close: f64) -> (f64, f64, f64) {
        let value = self.inner.append(close);
        self.stc.push(value.stc);
        self.macd.push(value.macd);
        self.stoch.push(value.stoch);
        (value.stc, value.macd, value.stoch)
    }

    fn extend(&mut self, close: PyReadonlyArray1<f64>) -> PyResult<()> {
        for &close in close.as_slice()? {
            self.append(close);
        }
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
            PyArray1::from_vec(py, self.stc.clone()),
            PyArray1::from_vec(py, self.macd.clone()),
            PyArray1::from_vec(py, self.stoch.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.stc, value.macd, value.stoch))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.stc.clear();
        self.macd.clear();
        self.stoch.clear();
    }
}
