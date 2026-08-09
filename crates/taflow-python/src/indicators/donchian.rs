use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::stream::Donchian;

#[pyclass]
pub struct DonchianOperator {
    inner: Donchian,
    upper: Vec<f64>,
    lower: Vec<f64>,
    middle: Vec<f64>,
}

#[pymethods]
impl DonchianOperator {
    #[new]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: Donchian::new(timeperiod)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            upper: Vec::new(),
            lower: Vec::new(),
            middle: Vec::new(),
        })
    }
    fn append(&mut self, high: f64, low: f64) -> Option<(f64, f64, f64)> {
        let value = self.inner.append(high, low);
        if let Some(value) = value {
            self.upper.push(value.upper);
            self.lower.push(value.lower);
            self.middle.push(value.middle);
            Some((value.upper, value.lower, value.middle))
        } else {
            self.upper.push(f64::NAN);
            self.lower.push(f64::NAN);
            self.middle.push(f64::NAN);
            None
        }
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (high, low) = (high.as_slice()?, low.as_slice()?);
        if high.len() != low.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "inputs must have equal lengths",
            ));
        }
        let inner = &mut self.inner;
        let upper = &mut self.upper;
        let lower = &mut self.lower;
        let middle = &mut self.middle;
        py.allow_threads(|| inner.extend_slices_into(high, low, upper, lower, middle))
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
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
            PyArray1::from_vec(py, self.upper.clone()),
            PyArray1::from_vec(py, self.lower.clone()),
            PyArray1::from_vec(py, self.middle.clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner.value().map(|v| (v.upper, v.lower, v.middle))
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.upper.clear();
        self.lower.clear();
        self.middle.clear();
    }
    fn __len__(&self) -> usize {
        self.upper.len()
    }
}
