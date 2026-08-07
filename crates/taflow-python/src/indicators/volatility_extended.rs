use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{Donchian, UlcerIndex};
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
            inner: Donchian::new(timeperiod).map_err(|e| PyValueError::new_err(e.to_string()))?,
            upper: Vec::new(),
            lower: Vec::new(),
            middle: Vec::new(),
        })
    }
    fn append(&mut self, high: f64, low: f64) -> Option<(f64, f64, f64)> {
        let v = self.inner.append(high, low);
        if let Some(v) = v {
            self.upper.push(v.upper);
            self.lower.push(v.lower);
            self.middle.push(v.middle);
            Some((v.upper, v.lower, v.middle))
        } else {
            self.upper.push(f64::NAN);
            self.lower.push(f64::NAN);
            self.middle.push(f64::NAN);
            None
        }
    }
    fn extend(&mut self, high: PyReadonlyArray1<f64>, low: PyReadonlyArray1<f64>) -> PyResult<()> {
        let (h, l) = (high.as_slice()?, low.as_slice()?);
        if h.len() != l.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        for (&h, &l) in h.iter().zip(l) {
            self.append(h, l);
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
}
#[pyclass]
pub struct UlcerIndexOperator {
    inner: UlcerIndex,
    outputs: Vec<f64>,
}
#[pymethods]
impl UlcerIndexOperator {
    #[new]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: UlcerIndex::new(timeperiod).map_err(|e| PyValueError::new_err(e.to_string()))?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, input: f64) -> Option<f64> {
        let v = self.inner.append(input);
        self.outputs.push(v.unwrap_or(f64::NAN));
        v
    }
    fn extend(&mut self, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        for &v in input.as_slice()? {
            self.append(v);
        }
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.outputs.clone())
    }
    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}
