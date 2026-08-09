use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{ChaikinVolatility, KeltnerChannels};
#[pyclass]
pub struct KeltnerChannelsOperator {
    inner: KeltnerChannels,
    upper: Vec<f64>,
    middle: Vec<f64>,
    lower: Vec<f64>,
}
#[pymethods]
impl KeltnerChannelsOperator {
    #[new]
    fn new(timeperiod: usize, multiplier: f64) -> PyResult<Self> {
        Ok(Self {
            inner: KeltnerChannels::new(timeperiod, multiplier)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            upper: Vec::new(),
            middle: Vec::new(),
            lower: Vec::new(),
        })
    }
    fn append(&mut self, high: f64, low: f64, close: f64) -> Option<(f64, f64, f64)> {
        let v = self.inner.append(high, low, close);
        if let Some(v) = v {
            self.upper.push(v.upper);
            self.middle.push(v.middle);
            self.lower.push(v.lower);
            Some((v.upper, v.middle, v.lower))
        } else {
            self.upper.push(f64::NAN);
            self.middle.push(f64::NAN);
            self.lower.push(f64::NAN);
            None
        }
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (h, l, c) = (high.as_slice()?, low.as_slice()?, close.as_slice()?);
        if h.len() != l.len() || h.len() != c.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        py.allow_threads(|| {
            for ((&h, &l), &c) in h.iter().zip(l).zip(c) {
                self.append(h, l, c);
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
            PyArray1::from_vec(py, self.upper.clone()),
            PyArray1::from_vec(py, self.middle.clone()),
            PyArray1::from_vec(py, self.lower.clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner.value().map(|v| (v.upper, v.middle, v.lower))
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.upper.clear();
        self.middle.clear();
        self.lower.clear();
    }
}
#[pyclass]
pub struct ChaikinVolatilityOperator {
    inner: ChaikinVolatility,
    outputs: Vec<f64>,
}
#[pymethods]
impl ChaikinVolatilityOperator {
    #[new]
    fn new(timeperiod: usize, roc_period: usize) -> PyResult<Self> {
        Ok(Self {
            inner: ChaikinVolatility::new(timeperiod, roc_period)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let v = self.inner.append(high, low);
        self.outputs.push(v.unwrap_or(f64::NAN));
        v
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (h, l) = (high.as_slice()?, low.as_slice()?);
        if h.len() != l.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        py.allow_threads(|| {
            for (&h, &l) in h.iter().zip(l) {
                self.append(h, l);
            }
        });
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
